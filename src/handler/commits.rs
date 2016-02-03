use super::base::*;

use commit_tree::CommitTree;

pub struct Commits;

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    Ok(Html(Wrapper(RepositoryWrapper(&context, render::Commits(CommitTree::new(&context.repository))))).into())
  }
}

impl Route for Commits {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/commits".into()
  }
}
