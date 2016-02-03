use super::base::*;

pub struct Reviews;

impl Handler for Reviews {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let mut reviews: Vec<_> = context.appraised.all_reviews().map(|revs| revs.collect()).unwrap_or(Vec::new());
    reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
    reviews.reverse();
    Ok(Html(Wrapper(RepositoryWrapper(&context, &render::Reviews(&reviews)))).into())
  }
}

impl Route for Reviews {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/reviews".into()
  }
}
