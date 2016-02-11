use super::base::*;

pub struct Tree;

impl Handler for Tree {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let reff = itry!(router.find("ref").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let object = itry!(context.repository.revparse_single(reff), status::NotFound);
    let commit = itry!(object.as_commit().ok_or(Error::FromString("Object is not commit...")), status::InternalServerError);
    let tree = itry!(commit.tree(), status::InternalServerError);
    Ok(Html(Wrapper(RepositoryWrapper(&context, &render::RootTree(&("/".to_owned() + context.requested_path.to_str().unwrap() + "/tree/" + reff), &tree)))).into())
  }
}

impl Route for Tree {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/tree/:ref".into()
  }
}
