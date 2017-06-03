#![feature(conservative_impl_trait)]
#![feature(plugin)]
#![plugin(maud_macros)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]
#![allow(unknown_lints)]

extern crate ammonia;
extern crate chrono;
extern crate cookie;
extern crate crypto;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate git2;
extern crate git_appraise;
extern crate git_ship;
extern crate gravatar;
#[macro_use]
extern crate iron;
extern crate logger;
extern crate lru_time_cache;
extern crate maud;
#[macro_use]
extern crate mime;
// extern crate params;
extern crate pulldown_cmark;
extern crate reqwest;
extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate toml;
extern crate typemap;
extern crate unicase;
extern crate walkdir;
extern crate take;

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
mod config;
mod tree_entry;

use std::time::Duration;
use std::env;
use iron::prelude::*;
use router::*;
use logger::*;
use handler::Register;
use repository_context::inject_repository_context;

pub use repository_context::RepositoryContext;
pub use repository_extension::RepositoryExtension;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    let config = match config::load(env::args_os().nth(1).as_ref()) {
        Ok(config) => config,
        Err(err) => {
            println!("Failed to load config:\n{}", err);
            std::process::exit(1)
        },
    };

    println!("Running with config");
    println!("===================");
    println!("{}", toml::to_string(&config).unwrap());
    println!("===================");

    let mut router = Router::new();

    router
        .register(inject_repository_context(&config.repos.root, handler::Review))
        .register(inject_repository_context(&config.repos.root, handler::Reviews))
        .register(inject_repository_context(&config.repos.root, handler::Commit))
        .register(inject_repository_context(&config.repos.root, handler::Commits))
        .register(inject_repository_context(&config.repos.root, handler::Repository))
        .register(handler::Repositories { root: config.repos.root.clone() })
        .register(handler::Settings)
        .register(handler::SettingsPost)
        .register(handler::About)
        .register(inject_repository_context(&config.repos.root, handler::Tree))
        .register(inject_repository_context(&config.repos.root, handler::Blob))
        .register(inject_repository_context(&config.repos.root, handler::Pages))
        .register(inject_repository_context(&config.repos.root, handler::Compare))
        .register(inject_repository_context(&config.repos.root, handler::git_smart_http::Refs))
        .register(inject_repository_context(&config.repos.root, handler::git_smart_http::UploadPack))
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
            enable_gravatar: config.avatars.gravatar.enable,
            enable_cache: config.avatars.cache.enable,
            cache_capacity: config.avatars.cache.capacity,
            cache_time_to_live: Duration::from_secs(config.avatars.cache.ttl_seconds),
        }));

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_before(settings::Settings::default());
    chain.link_after(handler::error::Error);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
