use super::base::*;

use std::path::Path;

pub struct TreeEntry;

impl Handler for TreeEntry {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let reff = itry!(router.find("ref").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let entry_path = itry!(router.find("path").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let object = itry!(context.repository.revparse_single(reff), status::NotFound);
    let commit = itry!(object.as_commit().ok_or(Error::FromString("Object is not commit...")), status::InternalServerError);
    let tree = itry!(commit.tree(), status::InternalServerError);
    let entry = itry!(tree.get_path(Path::new(entry_path)), status::NotFound);
    let parent = "/".to_owned() + context.requested_path.to_str().unwrap() + "/tree/" + reff;
    Ok(Html {
      render: Wrapper(RepositoryWrapper(&context, &render::TreeEntry(&context.repository, &parent, Path::new(&("/".to_owned() + entry_path)), &entry))),
      etag: Some(utils::sha1_strs(&[&reff, &entry_path])),
      req: req,
    }.into())
  }
}

impl Route for TreeEntry {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/tree/:ref/*path".into()
  }
}
