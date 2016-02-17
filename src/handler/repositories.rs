use super::base::*;

use std::fs;
use std::path::PathBuf;
use git2::Repository;
use repository_tree::RepositoryTreeEntry;

pub struct Repositories {
  pub root: PathBuf,
}

fn get_repos(root: &PathBuf, dir: &PathBuf) -> Option<Vec<RepositoryTreeEntry>> {
  fs::read_dir(dir).ok()
    .and_then(|entries| {
      let result: Vec<_> = entries.filter_map(|entry| {
        entry.ok().and_then(|entry| {
          entry.file_name().into_string().ok().and_then(|filename| {
            entry.file_type().ok().and_then(|ty| {
              if ty.is_dir() || (ty.is_symlink() && fs::metadata(entry.path()).map(|meta| meta.is_dir()).unwrap_or(false)) {
                Repository::open(entry.path()).ok()
                  .map(|repo| {
                    let actual = fs::canonicalize(&entry.path()).unwrap_or(entry.path());
                    if actual == entry.path() {
                      RepositoryTreeEntry::Repo(filename.clone(), repo)
                    } else {
                      let actual = actual.strip_prefix(root).ok().and_then(|stripped| stripped.to_str().map(|s| s.to_owned()));
                      match actual {
                        Some(actual) => RepositoryTreeEntry::Alias(filename.clone(), actual),
                        None => RepositoryTreeEntry::Repo(filename.clone(), repo),
                      }
                    }
                  })
                  .or_else(|| get_repos(root, &entry.path()).map(|repos| RepositoryTreeEntry::Dir(filename, repos)))
              } else {
                None
              }
            })
          })
        })
      }).collect();
      if result.is_empty() {
        None
      } else {
        Some(result)
      }
    })
}

impl Handler for Repositories {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let root = fs::canonicalize(&self.root).unwrap_or(self.root.clone());
    let repos = get_repos(&root, &root).unwrap_or_default();
    Ok(Html {
      render: Wrapper(&render::Repositories("", &repos)),
      etag: None,
      req: req,
    }.into())
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
