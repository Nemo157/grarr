use handler::base::*;

#[derive(Clone)]
pub struct Head;

impl Handler for Head {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);

    let head = itry!(
      context.repository
        .find_reference("HEAD")
        .map_err(Error::from)
        .and_then(|head|
          head.symbolic_target()
            .ok_or(Error::from("HEAD should be a symbolic ref"))
            .map(|target| format!("ref: {}", target))),
      status::InternalServerError);

    Ok(Response::with((status::Ok, mime!(Text/Plain; Charset=Utf8), head)))
  }
}

impl Route for Head {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/HEAD".into()
  }
}
