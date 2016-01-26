use git2::Repository;

pub enum RepositoryTreeEntry {
  Dir(String, Vec<RepositoryTreeEntry>),
  Repo(String, Repository),
  Alias(String, String),
}
