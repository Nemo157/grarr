use super::base::*;

use git2;
use walkdir::{ WalkDir, DirEntry, WalkDirIterator };

use std::path::{ Path, PathBuf };

#[derive(Clone)]
pub struct Repositories {
  pub root: PathBuf,
}

fn get_repo(root: &Path, dir: DirEntry) -> Option<(String, git2::Repository)> {
  let path = dir.path();
  let relative_dir = expect!(path.strip_prefix(root).ok());
  let relative = expect!(relative_dir.to_str()).to_owned();
  let repo = expect!(git2::Repository::open(&path).ok());
  Some((relative, repo))
}

fn get_repos(root: &Path) -> Vec<(String, git2::Repository)> {
  let mut repos = Vec::new();
  let mut it = WalkDir::new(root).into_iter();
  loop {
    let entry = match it.next() {
      None => break,
      Some(Err(_)) => continue,
      Some(Ok(entry)) => entry,
    };
    if entry.file_type().is_dir() {
      if let Some(repo) = get_repo(root, entry) {
        repos.push(repo);
        it.skip_current_dir();
      }
    }
  }
  repos
}

impl Handler for Repositories {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let repos = get_repos(&self.root);
    Html {
      render: render::Repositories(repos),
      etag: None,
      req: req,
    }.into()
  }
}

impl Route for Repositories {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/".into()
  }
}
