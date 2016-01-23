use std::path::PathBuf;
use iron::IronResult;
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;
use mime::Mime;
use super::route::Route;
use router::Router;
use iron::status;
use iron::method::Method;

use render;
use git_appraise::{ Oid, Repository };

pub struct Review {
  pub root: PathBuf,
}

impl Handler for Review {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("commit_id").unwrap()).unwrap();
    let review = repo.review_for(id).unwrap();
    let buffer = to_string!(#(render::Wrapper(render::ReviewRenderer(&review))));
    let mime: Mime = "text/html".parse().unwrap();
    Ok(Response::with((status::Ok, mime, buffer)))
  }
}

impl Route for Review {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/reviews/:commit_id"
  }
}
