use std::borrow::Cow;
use git2;

pub struct ReferencedCommit<'a> {
  pub commit: git2::Commit<'a>,
  pub reference: Option<git2::Reference<'a>>,
}

impl<'a> ReferencedCommit<'a> {
  pub fn shorthand_or_id(&self) -> Cow<str> {
    match self.reference.as_ref().and_then(|r| r.shorthand()) {
      Some(reff) => reff.into(),
      None => self.commit.id().to_string().into(),
    }
  }
}
