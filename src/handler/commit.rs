use super::base::*;

use error::Error;
use std::path::PathBuf;
use router::Router;
use git2::{ Oid, Repository };
use render::CommitRenderer;

pub struct Commit {
  pub root: PathBuf,
}

impl Handler for Commit {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let path = itry!(router.find("repo").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let repo = itry!(Repository::open(self.root.join(path)), status::NotFound);
    let commit = itry!(router.find("commit").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let id = itry!(Oid::from_str(commit), status::BadRequest);
    let commit = itry!(repo.find_commit(id), status::NotFound);
    Ok(Html(Wrapper(&CommitRenderer(&commit))).into())
  }
}

impl Route for Commit {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/commits/:commit"
  }
}
