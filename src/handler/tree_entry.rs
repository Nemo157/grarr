use iron::{ status, IronResult };
use std::path::Path;
use repository_context::RepositoryContext;
use git2;

pub struct TreeEntryContext<'a> {
  pub entry: git2::Object<'a>,
  pub parent: String,
}

pub fn get_tree_entry<'a>(context: &'a RepositoryContext, path: &str) -> IronResult<TreeEntryContext<'a>> {
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
  let reff = referenced_commit.reference.as_ref().and_then(|r| r.shorthand()).unwrap_or(&*idstr);
  let parent = "/".to_owned() + &context.requested_path.to_string_lossy()  + "/tree/" + reff;
  Ok(TreeEntryContext {
    entry: entry,
    parent: parent,
  })
}
