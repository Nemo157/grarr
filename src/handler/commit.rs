use super::base::*;

use git2::Oid;

#[derive(Clone)]
pub struct Commit;

impl Handler for Commit {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let commit = itry!(context.commit(), status::NotFound);
    Html {
      render: Wrapper(RepositoryWrapper(&context, &render::Commit(&("/".to_owned() + context.requested_path.to_str().unwrap()), &context.repository, &commit))),
      etag: Some(EntityTag::weak(versioned_sha1!())),
      req: req,
    }.into()
  }
}

impl Route for Commit {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/commit/:ref".into()
  }
}
