use super::base::*;

use std::fs;
use std::path::PathBuf;
use router::Router;
use git_appraise::{ Oid, Repository };
use render::ReviewRenderer;

pub struct Review {
  pub root: PathBuf,
}

impl Handler for Review {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let actual = fs::canonicalize(self.root.join(path)).unwrap().strip_prefix(&fs::canonicalize(&self.root).unwrap()).unwrap().to_str().unwrap().to_string();
    let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("commit_id").unwrap()).unwrap();
    let review = repo.review_for(id).unwrap();
    Ok(Html(Wrapper(RepositoryWrapper(&*path, &actual, Tab::Reviews, &ReviewRenderer(&review)))).into())
  }
}

impl Route for Review {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/reviews/:commit_id"
  }
}
