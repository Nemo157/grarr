use super::base::*;

use std::fs;
use std::path::PathBuf;
use router::Router;
use git_appraise::Repository;

use error::Error;
use render::ReviewsRenderer;

pub struct Reviews {
  pub root: PathBuf,
}

impl Handler for Reviews {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let path = itry!(router.find("repo").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let actual = fs::canonicalize(self.root.join(path)).unwrap().strip_prefix(&fs::canonicalize(&self.root).unwrap()).unwrap().to_str().unwrap().to_string();
    let repo = itry!(Repository::open(self.root.join(path)), status::NotFound);
    let mut reviews: Vec<_> = repo.all_reviews().map(|revs| revs.collect()).unwrap_or(Vec::new());
    reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
    Ok(Html(Wrapper(&(RepositoryWrapper(&*path, &actual, &Tab::Reviews, &ReviewsRenderer(&reviews))))).into())
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
