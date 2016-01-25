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

#[macro_use]
mod render;
mod handler;
mod error;

use std::env;
use iron::prelude::*;
use router::*;
use logger::*;
use handler::Register;
use time::Duration;

fn main() {
  let root = env::args().nth(1).unwrap();

  let mut router = Router::new();

  router
    .register(handler::Review { root: From::from(root.clone()) })
    .register(handler::Reviews { root: From::from(root.clone()) })
    .register(handler::Commit { root: From::from(root.clone()) })
    .register(handler::Commits { root: From::from(root.clone()) })
    .register(handler::Repository { root: From::from(root.clone()) })
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
