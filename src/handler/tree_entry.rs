use super::base::*;

use std::path::Path;
use router::Router;

pub struct TreeEntry;

impl Handler for TreeEntry {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let entry_path = itry!(router.find("path").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let head = itry!(context.repository.head().and_then(|head| head.resolve()), status::InternalServerError);
    let head_id = head.target().unwrap();
    let commit = itry!(context.repository.find_commit(head_id), status::InternalServerError);
    let tree = itry!(commit.tree(), status::InternalServerError);
    let entry = itry!(tree.get_path(Path::new(entry_path)), status::NotFound);
    let parent = "/".to_string() + context.requested_path.to_str().unwrap() + "/tree";
    Ok(Html(Wrapper(RepositoryWrapper(context.requested_path.to_str().unwrap(), context.canonical_path.to_str().unwrap(), &render::TreeEntry(&context.repository, &parent, Path::new(&("/".to_string() + entry_path)), &entry)))).into())
  }
}

impl Route for TreeEntry {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/tree/*path".into()
  }
}
