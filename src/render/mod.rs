#![allow(non_snake_case)]

#[macro_use]
mod macros;

mod style;
mod event;
mod ci_status;
mod review;
mod request;
mod comment;
mod analysis;
mod avatar;
mod commit;
mod repository;
mod repository_wrapper;
pub mod error;
mod fa;
mod tree;
mod wrapper;
mod signature;
mod diff;
mod utils;
mod highlight;
mod settings;
mod about;
mod reference;
mod compare;

use std::fmt;
use take::Take;

struct MovableArguments<F>(Take<F>) where F: FnOnce(&mut fmt::Formatter) -> fmt::Result;

impl<F> fmt::Display for MovableArguments<F> where F: FnOnce(&mut fmt::Formatter) -> fmt::Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.take()(f)
    }
}

pub use self::style::Style;
pub use self::event::{ Event, Events };
pub use self::request::{ Request, RequestStub };
pub use self::review::{ Review, Reviews };
pub use self::comment::{ Comment };
pub use self::ci_status::{ CIStatus };
pub use self::analysis::{ Analysis };
pub use self::avatar::{ Avatar };
pub use self::commit::{ Commit, CommitStub, Commits };
pub use self::repository::{ Repository, Repositories, RepositoryHeader };
pub use self::repository_wrapper::{ RepositoryWrapper, Tab };
pub use self::error::{ Error };
pub use self::tree::{ TreeEntry, Tree, Blob };
pub use self::wrapper::{wrapper, Wrapper};
pub use self::signature::Signature;
pub use self::diff::{ DiffCommit, DiffCommits };
pub use self::utils::MaybeLink;
pub use self::highlight::HighlightJS;
pub use self::settings::Settings;
pub use self::about::about;
pub use self::reference::Reference;
pub use self::compare::Compare;
