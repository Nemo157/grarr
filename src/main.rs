#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![plugin(maud_macros)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

extern crate maud;
#[macro_use]
extern crate iron;
extern crate router;
extern crate logger;
extern crate git2;
extern crate git_appraise;
extern crate typemap;
extern crate chrono;
extern crate maud_pulldown_cmark;
extern crate gravatar;
extern crate hyper;
#[macro_use]
extern crate mime;
extern crate lru_time_cache;
extern crate time;
extern crate pulldown_cmark;
extern crate crypto;
extern crate unicase;
extern crate walkdir;
extern crate params;

#[macro_use]
mod macros;

#[macro_use]
pub mod render;
#[macro_use]
pub mod handler;
mod error;
mod commit_tree;
mod repository_context;
mod repository_extension;
mod settings;
mod referenced_commit;

use std::env;
use std::path::Path;
use iron::prelude::*;
use router::*;
use logger::*;
use handler::Register;
use time::Duration;
use repository_context::inject_repository_context;

pub use repository_context::RepositoryContext;
pub use repository_extension::RepositoryExtension;

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
    .register(handler::Settings)
    .register(handler::SettingsPost)
    .register(handler::About)
    // .register(inject_repository_context(Path::new(&root), handler::Tree))
    .register(inject_repository_context(Path::new(&root), handler::TreeEntry))
    .register(statics![
      prefix: "./static/";
      "./static/js/highlight.js",
      "./static/css/highlight-solarized-light.css",
      "./static/css/layout.css",
      "./static/css/theme-solarized-dark.css",
      "./static/css/theme-solarized-light.css",
      "./static/css/font-awesome.min.css",
      "./static/fonts/FontAwesome.otf",
      "./static/fonts/fontawesome-webfont.eot",
      "./static/fonts/fontawesome-webfont.svg",
      "./static/fonts/fontawesome-webfont.ttf",
      "./static/fonts/fontawesome-webfont.woff",
      "./static/fonts/fontawesome-webfont.woff2",
    ])
    .register(handler::Avatars::new(handler::avatar::Options {
      enable_gravatar: true,
      enable_cache: true,
      cache_capacity: 100,
      cache_time_to_live: Duration::minutes(1),
    }));

  let (logger_before, logger_after) = Logger::new(None);

  let mut chain = Chain::new(router);

  chain.link_before(logger_before);
  chain.link_before(settings::Settings::default());
  chain.link_after(handler::error::NotFound);
  chain.link_after(handler::error::BadRequest);
  chain.link_after(handler::error::InternalServerError);
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:3000").unwrap();
}
