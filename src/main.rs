#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
#[macro_use]
extern crate iron;
extern crate router;
extern crate logger;
extern crate git2;
extern crate git_appraise;
extern crate persistent;
extern crate typemap;
extern crate chrono;
extern crate maud_pulldown_cmark;
extern crate gravatar;
extern crate hyper;
extern crate mime;
extern crate lru_time_cache;
extern crate time;
extern crate walkdir;
extern crate pulldown_cmark;

#[macro_use]
mod macros;

#[macro_use]
mod render;
mod handler;
mod error;
mod repository_tree;
mod commit_tree;
mod repository_context;

use std::env;
use std::path::Path;
use iron::prelude::*;
use router::*;
use logger::*;
use handler::Register;
use time::Duration;
use repository_context::inject_repository_context;

fn main() {
  let root = env::args().nth(1).unwrap();

  let mut router = Router::new();

  router
    .register(inject_repository_context(Path::new(&root), handler::Review))
    .register(inject_repository_context(Path::new(&root), handler::Reviews))
    .register(inject_repository_context(Path::new(&root), handler::Commit))
    .register(inject_repository_context(Path::new(&root), handler::Commits))
    .register(inject_repository_context(Path::new(&root), handler::Repository))
    .register(handler::Repositories { root: root.clone().into() })
    .register(inject_repository_context(Path::new(&root), handler::Tree))
    .register(inject_repository_context(Path::new(&root), handler::TreeEntry))
    .register(handler::Avatars::new(handler::avatar::Options {
      enable_gravatar: true,
      enable_cache: true,
      cache_capacity: 100,
      cache_time_to_live: Duration::minutes(1),
    }));

  let (logger_before, logger_after) = Logger::new(None);

  let mut chain = Chain::new(router);

  chain.link_before(logger_before);
  chain.link_after(handler::NotFound);
  chain.link_after(handler::BadRequest);
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:3000").unwrap();
}
