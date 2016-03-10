use super::base::*;
use tree_entry;

use git2;
use std::path::Path;

#[derive(Clone)]
pub struct Tree;

impl Handler for Tree {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let entry = try!(tree_entry::get_tree_entry(&context, router.find("path").unwrap_or("")));
    match entry.entry.kind() {
      Some(git2::ObjectType::Tree) => {
        Html {
          render: RepositoryWrapper(&context, &render::Tree(entry.entry.as_tree().unwrap(), &entry)),
          etag: Some(EntityTag::weak(versioned_sha1!(&entry.commit.commit.id()))),
          req: req,
        }.into()
      },
      Some(git2::ObjectType::Blob) => {
        let new_url = Url::parse(&*req.url.to_string().replace("tree", "blob")).unwrap();
        Ok(Response::with((status::TemporaryRedirect, Redirect(new_url))))
      },
      other => {
        Err(IronError::new(Error::from(format!("Can only handle blobs and trees, not {:?}", other)), status::InternalServerError))
      },
    }
  }
}

impl Route for Tree {
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
