use iron::{ status, IronResult };
use std::path::Path;
use repository_context::RepositoryContext;
use git2;
use referenced_commit::ReferencedCommit;

pub struct TreeEntryContext<'a> {
  pub entry: git2::Object<'a>,
  pub repo_path: &'a str,
  pub entry_path: String,
  pub reff: String,
  pub commit: ReferencedCommit<'a>,
}

impl<'a> TreeEntryContext<'a> {
  pub fn extension(&self) -> Option<&str> {
    self.entry_path
      .rfind('.')
      .and_then(|i|
        if i + 1 == self.entry_path.len() || (&self.entry_path[i+1..]).contains('/') {
          None
        } else {
          Some(&self.entry_path[i+1..])
        })
  }

  pub fn parent(&self) -> Option<&str> {
    if self.entry_path == "/" {
      None
    } else {
      Some(&self.entry_path[..self.entry_path.rfind('/').unwrap_or(0)])
    }
  }
}

pub fn get_tree_entry<'a>(context: &'a RepositoryContext, path: &'a str) -> IronResult<TreeEntryContext<'a>> {
  let referenced_commit = itry!(context.referenced_commit(), status::NotFound);
  let tree = itry!(referenced_commit.commit.tree(), status::InternalServerError);
  let entry = if path == "" {
    itry!(context.repository.find_object(tree.id(), Some(git2::ObjectType::Tree)))
  } else {
    let tree_entry = itry!(tree.get_path(Path::new(path)), status::NotFound);
    itry!(tree_entry.to_object(&context.repository), status::InternalServerError)
  };
  let id = referenced_commit.commit.id();
  let idstr = format!("{}", id);
  let reff = referenced_commit.shorthand_or_id().into_owned();
  Ok(TreeEntryContext {
    entry: entry,
    repo_path: &context.path,
    entry_path: "/".to_owned() + path,
    reff: reff,
    commit: referenced_commit,
  })
}
