use super::base::*;

use std::path::Path;

#[derive(Clone)]
pub struct TreeEntry;

impl Handler for TreeEntry {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let entry_path = router.find("path").unwrap_or("");
    let referenced_commit = itry!(context.referenced_commit(), status::NotFound);
    let tree = itry!(referenced_commit.commit.tree(), status::InternalServerError);
    let obj;
    let entry;
    if entry_path == "" {
      entry = tree.as_object();
    } else {
      let tree_entry = itry!(tree.get_path(Path::new(entry_path)), status::NotFound);
      obj = itry!(tree_entry.to_object(&context.repository), status::InternalServerError);
      entry = &obj;
    }
    let id = referenced_commit.commit.id();
    let idstr = format!("{}", id);
    let reff = referenced_commit.reference.as_ref().and_then(|r| r.shorthand()).unwrap_or(&*idstr);
    let parent = "/".to_owned() + &context.requested_path.to_string_lossy()  + "/tree/" + reff;
    Html {
      render: RepositoryWrapper(&context, &render::TreeEntry(&parent, Path::new(entry_path), entry, &referenced_commit)),
      etag: Some(EntityTag::weak(versioned_sha1!(&id))),
      req: req,
    }.into()
  }
}

impl Route for TreeEntry {
  fn method() -> Method {
    Method::Get
  }

  fn routes() -> Vec<Cow<'static, str>> {
    vec![
      "/tree/:ref".into(),
      "/tree/:ref/*path".into(),
    ]
  }
}
