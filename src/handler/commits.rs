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
use git2::{ Repository };

pub struct Commits {
  pub root: PathBuf,
}

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let mut walker = repo.revwalk().unwrap();
    walker.push_head().unwrap();
    let commits: Vec<_> = walker.map(|id| repo.find_commit(id).unwrap()).collect();
    let buffer = to_string!(#(render::Wrapper(render::CommitsRenderer(&commits))));
    let mime: Mime = "text/html".parse().unwrap();
    Ok(Response::with((status::Ok, mime, buffer)))
  }
}

impl Route for Commits {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/commits"
  }
}
