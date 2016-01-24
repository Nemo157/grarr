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
use git2::Repository as GitRepository;

pub struct Repository {
  pub root: PathBuf,
}

impl Handler for Repository {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = GitRepository::open(self.root.join(path)).unwrap();
    let buffer = to_string!(#(render::Wrapper(render::RepositoryRenderer(&*path, &repo))));
    let mime: Mime = "text/html".parse().unwrap();
    Ok(Response::with((status::Ok, mime, buffer)))
  }
}

impl Route for Repository {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo"
  }
}
