use handler::base::*;
use super::utils::*;

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

#[derive(Clone)]
pub struct UploadPack;

#[derive(Debug, Eq, PartialEq)]
enum Capability {
  SideBand,
  SideBand64K,
  Unknown(String),
}

#[derive(Debug)]
struct UploadPackRequest {
  wants: Vec<Oid>,
  haves: Vec<Oid>,
  capabilities: Vec<Capability>,
  done: bool,
  context: RepositoryContext,
}

struct UploadPackContext<'a> {
  repository: &'a Repository,
  refs: HashSet<Oid>,
}

enum UploadPackResponse<'a> {
  Pack(Revwalk<'a>),
  /* Continue( ... ), */
}

fn parse_request(req: &mut Request, context: RepositoryContext) -> Result<UploadPackRequest, Error> {
  let mut request = UploadPackRequest {
    wants: Vec::new(),
    haves: Vec::new(),
    capabilities: Vec::new(),
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
  for line in body.pkt_lines() {
    let line = try!(line);
    if line.len() < 4 { continue }
    match &line[0..4] {
      "want" => {
        let line = line[5..].trim();
        let (want, caps) = line.split_at(line.find(' ').unwrap_or(line.len()));
        request.wants.push(try!(want.parse()));
        request.capabilities.extend(caps.split(' ').map(Capability::from));
      },
      "have" => {
        let end = line.find(|c| c == '\n' || c == '\0').unwrap_or(line.len());
        request.haves.push(try!(line[5..end].parse()));
      },
      "done" => {
        request.done = true;
        break;
      },
      _ => return Err(Error::from(format!("Unexpected pkt-line {}", line))),
    }
  }
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
      return Ok(false);
    }
  }
  Ok(true)
}

#[allow(collapsible_if)]
fn compute_response<'a>(context: &'a UploadPackContext, request: &'a UploadPackRequest) -> Result<UploadPackResponse<'a>, Error> {
  let mut common = HashSet::<Oid>::new();
  // for each id given in have
  for id in request.haves.iter().cloned() {
    // if it is an ancestor of a ref
    if try!(graph_ancestor_of_any(&context.repository, id, context.refs.iter().cloned())) {
      // and is not an ancestor of a common
      if !try!(graph_ancestor_of_any(&context.repository, id, common.iter().cloned())) {
        // add it to common
        common.insert(id);
      }
    }
  }
  if request.done || try!(is_closed(&context.repository, request.wants.iter().cloned(), common.iter().cloned())) {
    let mut walker = try!(context.repository.revwalk());
    for id in request.wants.iter().cloned() {
      try!(walker.push(id));
    }
    for id in common.iter().cloned() {
      try!(walker.hide(id));
    }
    Ok(UploadPackResponse::Pack(walker))
  } else {
    Err(Error::from("TODO: ......"))
  }
}

fn build_pack<'a>(repository: &'a Repository, mut revwalk: Revwalk<'a>, output: Multiplexer) -> Result<(), Error> {
  let output = Rc::new(Mutex::new(output));
  let mut builder = try!(repository.packbuilder());
  {
    let output = output.clone();
    let mut first_delta = true;
    try!(builder.set_progress_callback(move |stage, current, total| {
      let mut output = output.lock().unwrap();
      match stage {
        PackBuilderStage::AddingObjects => {
          let _ = write!(output.progress(), "Counting objects {}\r", current);
        }
        PackBuilderStage::Deltafication => {
          if first_delta {
            let _ = write!(output.progress(), "\n");
            first_delta = false;
          }
          let percent = (current as f64 / total as f64) * 100.0;
          let _ = write!(
            output.progress(),
            "Compressing objects: {:.0}% ({}/{})",
            percent, current, total);
          if current == total {
            let _ = write!(output.progress(), ", done\n");
          } else {
            let _ = write!(output.progress(), "\r");
          }
        }
      }
      let _ = output.flush();
      true
    }));
  }
  try!(builder.insert_walk(&mut revwalk));
  let mut error = None;
  try!(builder.foreach(|object| {
    let mut output = output.lock().unwrap();
    match output.packfile().write_all(object) {
      err @ Err(_) => {
        error = Some(err);
        false
      }
      Ok(()) => true
    }
  }));
  if let Some(err) = error {
    try!(err);
  }
  try!(output.lock().unwrap().close());
  Ok(())
}

impl WriteBody for UploadPackRequest {
  fn write_body(&mut self, mut res: &mut Write) -> io::Result<()> {
    res.write_pkt_line("NAK")?;
    let limit = if self.capabilities.contains(&Capability::SideBand64K) {
      Some(65520)
    } else if self.capabilities.contains(&Capability::SideBand) {
      Some(1000)
    } else {
      None
    };
    let output = Multiplexer::new(res, limit);
    println!( "Preparing context for {}", self.context.path);
    let context2 = prepare_context(&self.context).unwrap();
    println!( "Prepared context for {}", self.context.path);
    validate_request(&context2, self).unwrap();
    println!( "Validated request for {}", self.context.path);
    let result = compute_response(&context2, self).unwrap();
    println!( "Computed response for {}", self.context.path);
    match result {
      UploadPackResponse::Pack(revwalk) => {
        build_pack(&self.context.repository, revwalk, output).unwrap();
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

impl<S: AsRef<str>> From<S> for Capability {
  fn from(s: S) -> Capability {
    match s.as_ref() {
      "side-band" => Capability::SideBand,
      "side-band-64k" => Capability::SideBand64K,
      s => Capability::Unknown(s.to_owned()),
    }
  }
}
