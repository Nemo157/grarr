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
use git_appraise::{ Repository };

pub struct Reviews {
  pub root: PathBuf,
}

impl Handler for Reviews {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let mut reviews: Vec<_> = repo.all_reviews().unwrap().collect();
    reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
    let buffer = to_string!(#(render::Wrapper(render::ReviewsRenderer(&reviews))));
    let mime: Mime = "text/html".parse().unwrap();
    Ok(Response::with((status::Ok, mime, buffer)))
  }
}

impl Route for Reviews {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/reviews"
  }
}
