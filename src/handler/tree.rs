use super::base::*;

pub struct Tree;

impl Handler for Tree {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let head = itry!(context.repository.head().and_then(|head| head.resolve()), status::InternalServerError);
    let head_id = head.target().unwrap();
    let commit = itry!(context.repository.find_commit(head_id), status::InternalServerError);
    let tree = itry!(commit.tree(), status::InternalServerError);
    Ok(Html(Wrapper(RepositoryWrapper(context.requested_path.to_str().unwrap(), context.canonical_path.to_str().unwrap(), Tab::Files, &render::RootTree(&("/".to_string() + context.requested_path.to_str().unwrap() + "/tree"), &tree)))).into())
  }
}

impl Route for Tree {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/tree".into()
  }
}
