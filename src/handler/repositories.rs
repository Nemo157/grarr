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
    match $expr {
      ::std::option::Option::Some(x) => x,
      ::std::option::Option::None => return None,
    }
  })
}

macro_rules! texpect {
  ($expr:expr) => ({
    match $expr {
      ::std::result::Result::Ok(x) => x,
      ::std::result::Result::Err(_) => return None,
    }
  })
}

fn get_repo(root: &PathBuf, dir: DirEntry) -> Option<(String, Repository)> {
  let path = dir.path();
  let relative_dir = texpect!(path.strip_prefix(root));
  let relative = expect!(relative_dir.to_str()).to_string();
  let repo = texpect!(Repository::open(&path));
  Some((relative, repo))
}

impl Handler for Repositories {
  fn handle(&self, _: &mut Request) -> IronResult<Response> {
    let mut repos = Vec::new();
    let mut it = WalkDir::new(&self.root)
      .min_depth(1)
      .max_depth(3)
      .follow_links(true)
      .into_iter();
    loop {
      let entry = match it.next() {
        None => break,
        Some(Err(err)) => continue,
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
