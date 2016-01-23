use iron::IronResult;
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;
use mime::Mime;
use super::route::Route;
use iron::status;
use iron::method::Method;

use render;
use git_appraise::{ Repository };

pub struct Reviews {
  pub repo: String,
}

impl Handler for Reviews {
  fn handle(&self, _: &mut Request) -> IronResult<Response> {
    let repo = Repository::open(&*self.repo).unwrap();
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
    "/reviews"
  }
}
