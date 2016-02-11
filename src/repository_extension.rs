use git2;
use RepositoryContext;

pub trait RepositoryExtension {
  fn origin_url(&self) -> Option<String>;
}

impl RepositoryExtension for git2::Repository {
  fn origin_url(&self) -> Option<String> {
    self.find_remote("origin").ok().and_then(|remote| remote.url().map(ToOwned::to_owned))
  }
}

impl RepositoryExtension for RepositoryContext {
  fn origin_url(&self) -> Option<String> {
    self.repository.origin_url()
  }
}
