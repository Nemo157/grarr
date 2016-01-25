use super::base::*;

use std::path::PathBuf;
use router::Router;
use git2::{ Oid, Repository };
use render::CommitRenderer;

pub struct Commit {
  pub root: PathBuf,
}

impl Handler for Commit {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = iexpect!(req.extensions.get::<Router>(), status::InternalServerError);
    let path = iexpect!(router.find("repo"), status::InternalServerError);
    let repo = itry!(Repository::open(self.root.join(path)), status::NotFound);
    let id = iexpect!(router.find("commit").and_then(|id| Oid::from_str(id).ok()));
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
