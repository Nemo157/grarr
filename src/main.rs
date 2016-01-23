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
extern crate mime;

#[macro_use]
mod render;
mod handler;

use std::env;
use iron::prelude::*;
use router::*;
use logger::*;
use handler::Register;

fn main() {
  let path = env::args().nth(1).unwrap();

  let mut router = Router::new();

  router
    .register(handler::Review { repo: path.clone() })
    .register(handler::Reviews { repo: path.clone() })
    .register(handler::Avatars { enable_gravatar: true });

  let (logger_before, logger_after) = Logger::new(None);

  let mut chain = Chain::new(router);

  chain.link_before(logger_before);
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:3000").unwrap();
}
