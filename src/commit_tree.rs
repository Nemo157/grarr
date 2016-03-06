use std::vec::IntoIter;
use git2::{ self, Oid, Repository, Commit };

pub struct CommitTree<'repo> {
  repo: &'repo Repository,
  next_after: Option<Commit<'repo>>,
  next: Option<Commit<'repo>>,
  commits: IntoIter<Commit<'repo>>,
  ignored: Vec<Oid>,
  len: usize,
}

impl<'repo> CommitTree<'repo> {
  pub fn new(repo: &'repo Repository, commit: &Commit<'repo>, limit: usize) -> Result<CommitTree<'repo>, git2::Error> {
    let mut walker = try!(repo.revwalk());
    try!(walker.push(commit.id()));
    walker.simplify_first_parent();
    let mut all_commits = walker.map(|id| id.and_then(|id| repo.find_commit(id)));
    let commits = try!((&mut all_commits).take(limit).collect());
    let next_after = all_commits.next().and_then(|c| c.ok());
    Ok(CommitTree::create(repo, commits, next_after, Vec::new()))
  }

  pub fn is_empty(&self) -> bool {
    self.next.is_none()
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn next_after(&self) -> Option<&Commit<'repo>> {
    self.next_after.as_ref()
  }

  fn between(repo: &'repo Repository, first: &Commit<'repo>, ignored: Vec<Oid>) -> CommitTree<'repo> {
    let mut walker = repo.revwalk().unwrap();
    for parent in first.parent_ids().skip(1) {
      walker.push(parent).unwrap();
    }
    for ignored in ignored.clone() {
      walker.hide(ignored).unwrap();
    }
    walker.simplify_first_parent();
    let commits = walker.map(|id| id.and_then(|id| repo.find_commit(id)).unwrap()).collect();
    CommitTree::create(repo, commits, None, ignored)
  }

  fn create(repo: &'repo Repository, commits: Vec<Commit<'repo>>, next_after: Option<Commit<'repo>>, ignored: Vec<Oid>) -> CommitTree<'repo> {
    let len = commits.len();
    let mut iter = commits.into_iter();
    CommitTree {
      repo: repo,
      next_after: next_after,
      next: iter.next(),
      commits: iter,
      ignored: ignored,
      len: len,
    }
  }
}

impl<'repo> Iterator for CommitTree<'repo> {
  type Item = (Commit<'repo>, CommitTree<'repo>);

  fn next(&mut self) -> Option<Self::Item> {
    match self.next.take() {
      Some(commit) => {
        self.next = self.commits.next();
        let mut ignored = self.ignored.clone();
        if self.next.is_some() {
          ignored.push(self.next.as_ref().unwrap().id());
        }
        let sub = CommitTree::between(self.repo, &commit, ignored);
        self.len = self.len - 1;
        Some((commit, sub))
      },
      None => None,
    }
  }
}
