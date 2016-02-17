use super::base::*;

use git2::Oid;
use git_appraise::AppraisedRepository;

#[derive(Clone)]
pub struct Review;

impl Handler for Review {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let commit = itry!(router.find("commit").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let id = itry!(Oid::from_str(commit), status::BadRequest);
    let review = itry!(context.repository.review_for(id), status::NotFound);
    Html {
      render: Wrapper(RepositoryWrapper(&context, &render::Review(&review))),
      etag: None,
      req: req,
    }.into()
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
