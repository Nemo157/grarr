use git2;

pub struct ReferencedCommit<'a> {
  pub commit: git2::Commit<'a>,
  pub reference: Option<git2::Reference<'a>>,
}
