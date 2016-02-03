use super::base::*;

use render::RepositoryRenderer;

pub struct Repository;

impl Handler for Repository {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    Ok(Html(Wrapper(RepositoryWrapper(context.requested_path.to_str().unwrap(), context.canonical_path.to_str().unwrap(), Tab::Overview, &RepositoryRenderer(&context.repository)))).into())
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
