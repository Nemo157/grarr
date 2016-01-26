use super::base::*;

use std::fs;
use std::path::PathBuf;
use router::Router;
use git_appraise::Repository;

use render::ReviewsRenderer;

pub struct Reviews {
  pub root: PathBuf,
}

impl Handler for Reviews {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let path = req.extensions.get::<Router>().unwrap().find("repo").unwrap();
    let actual = fs::canonicalize(self.root.join(path)).unwrap().strip_prefix(&fs::canonicalize(&self.root).unwrap()).unwrap().to_str().unwrap().to_string();
    let repo = Repository::open(self.root.join(path)).unwrap();
    let mut reviews: Vec<_> = repo.all_reviews().unwrap().collect();
    reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
    Ok(Html(Wrapper(&ReviewsRenderer(&*path, &actual, &reviews))).into())
  }
}

impl Route for Reviews {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/reviews"
  }
}
