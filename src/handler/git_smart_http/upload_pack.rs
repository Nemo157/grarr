use handler::base::*;
use super::utils::*;

use std::io::{ Read, Write };
use std::collections::HashSet;

use iron::headers::{ CacheControl, CacheDirective, Vary, Pragma, Expires, HttpDate, ContentEncoding, Encoding };
use iron::modifiers::Header;
use unicase::UniCase;
use time;
use flate2::FlateReadExt;

use git2::{ self, Oid, Repository, Buf };

#[derive(Clone)]
pub struct UploadPack;

#[derive(Debug)]
struct UploadPackRequest {
  wants: Vec<Oid>,
  haves: Vec<Oid>,
  capabilities: Vec<String>,
  done: bool,
}

struct UploadPackContext<'a> {
  repository: &'a Repository,
  refs: HashSet<Oid>,
}

#[derive(Debug)]
enum UploadPackResponse {
  Pack(Vec<Oid>),
  /* Continue( ... ), */
}

fn parse_request(req: &mut Request) -> Result<UploadPackRequest, Error> {
  let mut request = UploadPackRequest {
    wants: Vec::new(),
    haves: Vec::new(),
    capabilities: Vec::new(),
    done: false,
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
        let end = line.find(|c| c == '\n' || c == '\0').unwrap_or(line.len());
        request.wants.push(try!(line[5..end].parse()));
        if let Some(nul) = line.find('\0') {
          request.capabilities.extend(line[nul..].trim().split(' ').map(ToOwned::to_owned));
        }
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
fn compute_response(context: &UploadPackContext, request: &UploadPackRequest) -> Result<UploadPackResponse, Error> {
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
    let commits = try!(walker.collect());
    Ok(UploadPackResponse::Pack(commits))
  } else {
    Err(Error::from("TODO: ......"))
  }
}

fn build_pack(repository: &Repository, commits: Vec<Oid>) -> Result<Vec<u8>, Error> {
  let mut builder = try!(repository.packbuilder());
  for id in commits {
    try!(builder.insert_commit(id));
  }
  let mut buf = Vec::new();
  try!(builder.foreach(|object| {
    buf.write_all(object).unwrap();
    true
  }));
  Ok(buf)
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
    let request = itry!(parse_request(req), (status::BadRequest, no_cache));
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
    let context2 = itry!(prepare_context(context), status::InternalServerError);
    itry!(validate_request(&context2, &request), status::BadRequest);
    let result = itry!(compute_response(&context2, &request), status::InternalServerError);
    match result {
      UploadPackResponse::Pack(commits) => {
        let pack = itry!(build_pack(&context.repository, commits), status::InternalServerError);
        println!("built {} byte pack", pack.len());
        let mut response = Vec::with_capacity(pack.len() + 8);
        response.write_pkt_line("NAK");
        response.write_all(&*pack);
        Ok(Response::with((status::Ok, no_cache, response)))
      },
    }
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
