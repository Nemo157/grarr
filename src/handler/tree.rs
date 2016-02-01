use super::base::*;

use std::fs;
use error::Error;
use std::path::PathBuf;
use router::Router;
use git2::{ Repository };
use render::RootTreeRenderer;

pub struct Tree {
  pub root: PathBuf,
}

impl Handler for Tree {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let path = itry!(router.find("repo").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let actual = fs::canonicalize(self.root.join(path)).unwrap().strip_prefix(&fs::canonicalize(&self.root).unwrap()).unwrap().to_str().unwrap().to_string();
    let repo = itry!(Repository::open(self.root.join(path)), status::NotFound);
    let head = itry!(repo.head().and_then(|head| head.resolve()), status::InternalServerError);
    let head_id = head.target().unwrap();
    let commit = itry!(repo.find_commit(head_id), status::InternalServerError);
    let tree = itry!(commit.tree(), status::InternalServerError);
    Ok(Html(Wrapper(RepositoryWrapper(&*path, &actual, Tab::Files, &RootTreeRenderer(&("/".to_string() + path + "/tree"), &tree)))).into())
  }
}

impl Route for Tree {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/*repo/tree"
  }
}
