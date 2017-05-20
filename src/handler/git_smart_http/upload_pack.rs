use handler::base::*;

use std::rc::Rc;
use std::sync::Mutex;
use std::io::{ self, Read, Write };
use std::collections::HashSet;

use iron::headers::{ CacheControl, CacheDirective, Vary, Pragma, Expires, HttpDate, ContentEncoding, Encoding };
use iron::modifiers::Header;
use iron::response::WriteBody;
use unicase::UniCase;
use time;
use flate2::FlateReadExt;

use git2::{ self, Oid, Repository, Revwalk, PackBuilderStage };
use git_ship::{pkt_line, PktLine, Multiplexer, Capability, Capabilities};

#[derive(Clone)]
pub struct UploadPack;

#[derive(Debug)]
struct UploadPackRequest {
    wants: Vec<Oid>,
    haves: Vec<Oid>,
    capabilities: Capabilities,
    done: bool,
    context: RepositoryContext,
}

struct UploadPackContext<'a> {
    repository: &'a Repository,
    refs: HashSet<Oid>,
}

enum UploadPackResponse<'a> {
    Pack {
        revwalk: Revwalk<'a>,
        common: Vec<Oid>,
    },
    Continue {
        common: Vec<Oid>,
    },
}

fn parse_request(req: &mut Request, context: RepositoryContext) -> Result<UploadPackRequest, Error> {
    let mut request = UploadPackRequest {
        wants: Vec::new(),
        haves: Vec::new(),
        capabilities: Capabilities::empty(),
        done: false,
        context: context,
    };
    let encoding = if let Some(&ContentEncoding(ref encodings)) = req.headers.get() {
        if encodings.len() != 1 {
            return Err(Error::from("Can't handle multiple encodings"));
        }
        encodings[0].clone()
    } else {
        Encoding::Identity
    };
    let mut body = match encoding {
        Encoding::Identity => Box::new(&mut req.body) as Box<Read>,
        Encoding::Gzip => Box::new(try!((&mut req.body).gz_decode())) as Box<Read>,
        Encoding::Deflate => Box::new((&mut req.body).deflate_decode()) as Box<Read>,
        encoding => return Err(Error::from(format!("Can't handle encoding {}", encoding))),
    };
    pkt_line::each_str(&mut body, |line| {
        println!("line {:?}", line);
        let line = match line {
            PktLine::Flush => return Ok(()),
            PktLine::Line(line) => line,
        };
        if line.len() < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unexpected pkt-line {}", line)));
        }
        match &line[0..4] {
            "want" => {
                let line = line[5..].trim();
                let (want, caps) = line.split_at(line.find(' ').unwrap_or(line.len()));
                request.wants.push(want.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?);
                if !caps.is_empty() {
                    request.capabilities = caps.parse().unwrap();
                }
            },
            "have" => {
                request.haves.push(line[5..].trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?);
            },
            "done" => {
                request.done = true;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unexpected pkt-line {}", line))),
        }
        Ok(())
    })?;
    println!("request: {:?}", request);
    Ok(request)
}

fn prepare_context(context: &RepositoryContext) -> Result<UploadPackContext, Error> {
    let mut refs = HashSet::new();
    for reff in try!(context.repository.references()) {
        let reff = try!(reff);
        let reff = try!(reff.resolve());
        refs.insert(try!(reff.target().ok_or("ref missing target")));
    }
    Ok(UploadPackContext { repository: &context.repository, refs: refs })
}

fn validate_request(context: &UploadPackContext, request: &UploadPackRequest) -> Result<(), Error> {
    if request.wants.is_empty() {
        return Err(Error::from("need wants"));
    }

    for id in &request.wants {
        if !context.refs.contains(&id) {
            return Err(Error::from(format!("want missing from refs {}", id)));
        }
    }

    Ok(())
}

fn graph_ancestor_of_any<I: Iterator<Item=Oid>>(repository: &Repository, commit: Oid, descendants: I) -> Result<bool, git2::Error> {
    for descendant in descendants {
        if try!(repository.graph_descendant_of(descendant, commit)) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn graph_descendant_of_any<I: Iterator<Item=Oid>>(repository: &Repository, commit: Oid, ancestors: I) -> Result<bool, git2::Error> {
    for ancestor in ancestors {
        if try!(repository.graph_descendant_of(commit, ancestor)) {
            return Ok(true);
        }
    }
    Ok(false)
}

// a commit set is closed if every commit in `descendants` has at least one ancestor in `ancestors`
fn is_closed<I1: Iterator<Item=Oid>, I2: Iterator<Item=Oid> + Clone>(repository: &Repository, descendants: I1, ancestors: I2) -> Result<bool, Error> {
    for descendant in descendants {
        if !try!(graph_descendant_of_any(repository, descendant, ancestors.clone())) {
            println!("{:?} has no ancestors in {:?}", descendant, ancestors.clone().collect::<Vec<_>>());
            return Ok(false);
        }
    }
    Ok(true)
}

#[allow(collapsible_if)]
fn compute_response<'a>(context: &'a UploadPackContext, request: &'a UploadPackRequest) -> Result<UploadPackResponse<'a>, Error> {
    let mut common = Vec::<Oid>::new();
    // for each id given in have
    for id in request.haves.iter().cloned() {
        // if it is an ancestor of a ref
        if try!(graph_ancestor_of_any(&context.repository, id, context.refs.iter().cloned())) {
            // and is not an ancestor of a common
            if !try!(graph_ancestor_of_any(&context.repository, id, common.iter().cloned())) {
                // add it to common
                common.push(id);
            }
        }
    }

    println!("common: {:?}", common);

    if request.done || try!(is_closed(&context.repository, request.wants.iter().cloned(), common.iter().cloned())) {
        let mut revwalk = try!(context.repository.revwalk());
        for id in request.wants.iter().cloned() {
            try!(revwalk.push(id));
        }
        for id in common.iter().cloned() {
            try!(revwalk.hide(id));
        }
        Ok(UploadPackResponse::Pack { revwalk: revwalk, common: common })
    } else {
        Ok(UploadPackResponse::Continue { common: common })
    }
}

fn build_pack<'a>(repository: &'a Repository, mut revwalk: Revwalk<'a>, mut output: Multiplexer) -> Result<(), Error> {
    {
        let output = Rc::new(Mutex::new(&mut output));
        let mut builder = repository.packbuilder()?;
        {
            let output = output.clone();
            let mut first_delta = true;
            builder.set_progress_callback(move |stage, current, total| {
                let mut output = output.lock().unwrap();
                match stage {
                    PackBuilderStage::AddingObjects => {
                        let _ = output.write_progress(format!("Counting objects {}\r", current));
                    }
                    PackBuilderStage::Deltafication => {
                        if first_delta {
                            let _ = output.write_progress("\n");
                            first_delta = false;
                        }
                        let percent = (current as f64 / total as f64) * 100.0;
                        if current == total {
                            let _ = output.write_progress(format!(
                                "Compressing objects: {:.0}% ({}/{}), done\n",
                                percent, current, total));
                        } else {
                            let _ = output.write_progress(format!(
                                "Compressing objects: {:.0}% ({}/{})\r",
                                percent, current, total));
                        }
                    }
                }
                true
            })?;
        }
        builder.insert_walk(&mut revwalk)?;
        builder.foreach(|object| output.lock().unwrap().write_packfile(object).is_ok())?;
    }
    pkt_line::flush(output.into_inner())?;
    Ok(())
}

impl WriteBody for UploadPackRequest {
    fn write_body(&mut self, mut res: &mut Write) -> io::Result<()> {
        let context2 = prepare_context(&self.context).unwrap();
        validate_request(&context2, self).unwrap();
        let result = compute_response(&context2, self).unwrap();
        match result {
            UploadPackResponse::Pack { revwalk, common } => {
                if !common.is_empty() && self.capabilities.contains(Capability::MultiAckDetailed) {
                    for id in &common {
                        let line = format!("ACK {} common", id);
                        println!("{}", line);
                        pkt_line::write_str(&mut res, line)?;
                    }
                    let line = format!("ACK {}", common.iter().last().unwrap());
                    println!("{}", line);
                    pkt_line::write_str(&mut res, line)?;
                } else {
                    pkt_line::write_str(&mut res, "NAK")?;
                }
                let output = Multiplexer::new(res, &self.capabilities)?;
                build_pack(&self.context.repository, revwalk, output).unwrap();
            },
            UploadPackResponse::Continue { common } => {
                if self.capabilities.contains(Capability::MultiAckDetailed) {
                    for id in common {
                        let line = format!("ACK {} common", id);
                        println!("{}", line);
                        pkt_line::write_str(&mut res, line)?;
                    }
                } else {
                    // TODO
                }
                pkt_line::write_str(&mut res, "NAK")?;
            },
        }
        Ok(())
    }
}

impl Handler for UploadPack {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let no_cache = (
            Header(CacheControl(vec![
                CacheDirective::NoCache,
                CacheDirective::MaxAge(0),
                CacheDirective::MustRevalidate,
            ])),
            Header(Expires(HttpDate(time::empty_tm()))),
            Header(Pragma::NoCache),
            Header(Vary::Items(vec![
                UniCase("accept-encoding".to_owned()),
            ])),
        );
        let context = itry!(req.extensions.remove::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
        let request = itry!(parse_request(req, context), (status::BadRequest, no_cache));
        println!("Prepared request for {}", request.context.path);
        Ok(Response::with((status::Ok, no_cache, Box::new(request) as Box<WriteBody>)))
    }
}

impl Route for UploadPack {
    fn method() -> Method {
        Method::Post
    }

    fn route() -> Cow<'static, str> {
        "/git-upload-pack".into()
    }
}
