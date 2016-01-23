#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate iron;
extern crate router;
extern crate logger;
extern crate git_appraise;
extern crate persistent;
extern crate typemap;
extern crate chrono;
extern crate maud_pulldown_cmark;
extern crate gravatar;
extern crate hyper;

#[macro_use]
mod render;

use gravatar::Gravatar;
use std::env;
use iron::prelude::*;
use iron::status;
use router::*;
use logger::*;
use iron::mime::Mime;
use git_appraise::{ Oid, Repository, Review };
use persistent::{ Read };
use typemap::Key;

fn get_reviews(repo: &Repository) -> Vec<Review> {
  let mut reviews: Vec<Review> = repo.all_reviews().unwrap().collect();
  reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
  reviews
}

fn get_review(repo: &Repository, id: Oid) -> Review {
  repo.review_for(id).unwrap()
}

pub fn result(buffer: String) -> IronResult<Response> {
  Ok(Response::with(("text/html".parse::<Mime>().unwrap(), status::Ok, buffer)))
}

fn reviews_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  let path = req.get::<Read<RepositoryPath>>().unwrap();
  let repo = Repository::open(&*path).unwrap();
  let reviews = get_reviews(&repo);
  result(to_string!(#(render::Wrapper(render::ReviewsRenderer(&reviews)))))
}

fn review_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  let path = req.get::<Read<RepositoryPath>>().unwrap();
  let repo = Repository::open(&*path).unwrap();
  let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("commit_id").unwrap()).unwrap();
  let review = get_review(&repo, id);
  result(to_string!(#(render::Wrapper(render::ReviewRenderer(&review)))))
}

fn avatars_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  use std::io::Read;
  let user = req.extensions.get::<Router>().unwrap().find("user").unwrap();
  let mut gravatar = Gravatar::new(user);
  gravatar.size = Some(30);
  gravatar.default = Some(gravatar::Default::Identicon);
  let client = hyper::client::Client::new();
  let mut res = client.get(&gravatar.image_url()).send().unwrap();
  assert_eq!(res.status, hyper::Ok);
  let mut buf = Vec::new();
  res.read_to_end(&mut buf).unwrap();
  let mime = res.headers.get::<iron::headers::ContentType>().unwrap().0.clone();
  Ok(Response::with((status::Ok, buf, mime)))
}

#[derive(Copy, Clone)]
struct RepositoryPath;
impl Key for RepositoryPath { type Value = String; }

fn main() {
  let path = env::args().nth(1).unwrap();

  let mut router = Router::new();
  router.get("/", reviews_handler);
  router.get("/avatars/:user", avatars_handler);
  router.get("/:commit_id", review_handler);

  let (logger_before, logger_after) = Logger::new(None);

  let mut chain = Chain::new(router);

  chain.link(Read::<RepositoryPath>::both(path));
  chain.link_before(logger_before);
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:3000").unwrap();
}
