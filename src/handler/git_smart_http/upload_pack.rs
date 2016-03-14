use handler::base::*;
use super::utils::*;

use git2::Oid;

#[derive(Clone)]
pub struct UploadPack;

#[derive(Debug)]
struct Details {
  wants: Vec<Oid>,
  haves: Vec<Oid>,
}

fn parse_requst(req: &mut Request) -> Result<Details, Error> {
  let mut details = Details { wants: Vec::new(), haves: Vec::new() };
  for line in req.body.pkt_lines() {
    let line = try!(line);
    if line.len() < 4 { continue }
    match &line[0..4] {
      "want" => {
        details.wants.push(try!(line[5..].parse()));
      },
      "have" => {
        details.haves.push(try!(line[5..].parse()));
      },
      "done" => break,
      _ => return Err(Error::from(format!("Unexpected pkt-line {}", line))),
    }
  }
  Ok(details)
}

impl Handler for UploadPack {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let details = parse_requst(req);
    // let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
    println!("{:?}", details);
    Err(IronError::new(Error::from("TODO"), status::InternalServerError))
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
