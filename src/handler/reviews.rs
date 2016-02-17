use super::base::*;
use git_appraise::AppraisedRepository;

pub struct Reviews;

impl Handler for Reviews {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let mut reviews: Vec<_> = context.repository.all_reviews().map(|revs| revs.collect()).ok().unwrap_or_default();
    reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
    reviews.reverse();
    Ok(Html {
      render: Wrapper(RepositoryWrapper(&context, &render::Reviews(&reviews))),
      etag: None,
      req: req,
    }.into())
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
