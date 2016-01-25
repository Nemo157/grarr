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
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("commit").unwrap()).unwrap();
    let commit = repo.find_commit(id).unwrap();
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
