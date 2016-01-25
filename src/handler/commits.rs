use super::base::*;

use std::path::PathBuf;
use router::Router;

use render::CommitsRenderer;
use git2::Repository;

pub struct Commits {
  pub root: PathBuf,
}

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let mut walker = repo.revwalk().unwrap();
    walker.push_head().unwrap();
    let commits: Vec<_> = walker.map(|id| repo.find_commit(id).unwrap()).collect();
    Ok(Html(Wrapper(&CommitsRenderer(&commits))).into())
  }
}

impl Route for Commits {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/commits"
  }
}
