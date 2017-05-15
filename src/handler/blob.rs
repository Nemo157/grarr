use super::base::*;
use tree_entry;

use git2;

#[derive(Clone)]
pub struct Blob;

impl Handler for Blob {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::from("missing extension")), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
    let path = router.find("path").unwrap_or("");
    let entry = try!(tree_entry::get_tree_entry(&context, path));
    let referenced_commit = itry!(context.referenced_commit(), status::NotFound);
    let id = referenced_commit.commit.id();
    match entry.entry.kind() {
      Some(git2::ObjectType::Blob) => {
        Html {
          render: RepositoryWrapper(&context, render::Blob(entry.entry.as_blob().unwrap(), &entry), Some(render::Tab::Files)),
          etag: Some(EntityTag::weak(versioned_sha1!(&id))),
          req: req,
        }.into()
      },
      Some(git2::ObjectType::Tree) => {
        let new_url = Url::parse(&*req.url.to_string().replace("blob", "tree")).unwrap();
        Ok(Response::with((status::TemporaryRedirect, Redirect(new_url))))
      },
      other => {
        Err(IronError::new(Error::from(format!("Can only handle blobs and trees, not {:?}", other)), status::InternalServerError))
      },
    }
  }
}

impl Route for Blob {
  fn method() -> Method {
    Method::Get
  }

  fn routes() -> Vec<Cow<'static, str>> {
    vec![
      "/blob/:ref".into(),
      "/blob/:ref/*path".into(),
    ]
  }
}
