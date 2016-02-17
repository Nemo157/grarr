use super::base::*;

pub struct Repository;

impl Handler for Repository {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let head_ref = itry!(context.repository.head(), status::InternalServerError);
    let resolved_head = itry!(head_ref.resolve(), status::InternalServerError);
    let head_id = itry!(resolved_head.target().ok_or(Error::FromString("Couldn't resolve head")), status::InternalServerError);
    Html {
      render: Wrapper(RepositoryWrapper(&context, &render::Repository(&context.repository, &head_id))),
      etag: Some(EntityTag::weak(sha1!(head_id.as_bytes()))),
      req: req,
    }.into()
  }
}

impl Route for Repository {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "".into()
  }
}
