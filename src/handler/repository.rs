use super::base::*;

use std::path::PathBuf;
use router::Router;
use git2::Repository as GitRepository;
use render::RepositoryRenderer;

pub struct Repository {
  pub root: PathBuf,
}

impl Handler for Repository {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = GitRepository::open(self.root.join(path)).unwrap();
    Ok(Html(Wrapper(&RepositoryRenderer(&*path, &repo))).into())
  }
}

impl Route for Repository {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo"
  }
}
