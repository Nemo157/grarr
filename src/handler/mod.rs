#[macro_use]
pub mod utils;
#[macro_use]
pub mod statics;

pub mod avatar;
pub mod route;
mod review;
mod reviews;
pub mod register;
mod commit;
mod commits;
mod repository;
mod repositories;
mod html;
mod base;
pub mod error;
mod settings;
mod about;
mod tree;
mod blob;
mod pages;
mod compare;
pub mod git_smart_http;

pub use self::avatar::Avatars;
pub use self::review::Review;
pub use self::reviews::Reviews;
pub use self::commit::Commit;
pub use self::commits::Commits;
pub use self::repository::Repository;
pub use self::repositories::Repositories;
pub use self::tree::Tree;
pub use self::blob::Blob;
pub use self::pages::Pages;
pub use self::compare::Compare;

pub use self::register::Register;
pub use self::statics::Static;
pub use self::settings::{ Settings, SettingsPost };
pub use self::about::About;
