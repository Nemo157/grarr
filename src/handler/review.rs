use super::base::*;

use router::Router;
use git_appraise::{ Oid };

pub struct Review;

impl Handler for Review {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let commit = itry!(router.find("commit").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let id = itry!(Oid::from_str(commit), status::BadRequest);
    let review = itry!(context.appraised.review_for(id), status::NotFound);
    Ok(Html(Wrapper(RepositoryWrapper(&context, &render::Review(&review)))).into())
  }
}

impl Route for Review {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/reviews/:commit".into()
  }
}
