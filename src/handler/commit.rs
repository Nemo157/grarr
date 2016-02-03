use super::base::*;

use router::Router;
use git2::{ Oid };
use render::CommitRenderer;

pub struct Commit;

impl Handler for Commit {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let commit = itry!(router.find("commit").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let id = itry!(Oid::from_str(commit), status::BadRequest);
    let commit = itry!(context.repository.find_commit(id), status::NotFound);
    Ok(Html(Wrapper(RepositoryWrapper(context.requested_path.to_str().unwrap(), context.canonical_path.to_str().unwrap(), Tab::Commits, &CommitRenderer(&commit)))).into())
  }
}

impl Route for Commit {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/commits/:commit".into()
  }
}
