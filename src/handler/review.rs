use super::base::*;

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
    let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("commit_id").unwrap()).unwrap();
    let review = repo.review_for(id).unwrap();
    Ok(Html(Wrapper(&ReviewRenderer(&review))).into())
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
