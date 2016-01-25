use super::base::*;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::path::PathBuf;
use git2::Repository;
use render::RepositoriesRenderer;

pub struct Repositories {
  pub root: PathBuf,
}

macro_rules! expect {
  ($expr:expr) => ({
    let result;
    match $expr {
      ::std::option::Option::Some(x) => { result = x; },
      ::std::option::Option::None => { return None; },
    }
    result
  })
}

fn get_repo(root: &PathBuf, dir: DirEntry) -> Option<(String, Repository)> {
  let path = dir.path();
  let relative_dir = expect!(path.strip_prefix(root).ok());
  let relative = expect!(relative_dir.to_str()).to_string();
  let repo = expect!(Repository::open(&path).ok());
  Some((relative, repo))
}

impl Handler for Repositories {
  fn handle(&self, _: &mut Request) -> IronResult<Response> {
    let mut repos = Vec::new();
    let mut it = WalkDir::new(&self.root).into_iter();
    loop {
      let entry = match it.next() {
        None => break,
        Some(Err(_)) => continue,
        Some(Ok(entry)) => entry,
      };
      if entry.file_type().is_dir() {
        if let Some(repo) = get_repo(&self.root, entry) {
          repos.push(repo);
          it.skip_current_dir();
        }
      }
    }
    Ok(Html(Wrapper(&RepositoriesRenderer(&repos))).into())
  }
}

impl Route for Repositories {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/"
  }
}
