use super::base::*;

use git2::Repository;
use commit_tree::CommitTree;

pub struct Commits;

fn render(path: &str, actual: &str, repo: &Repository) -> IronResult<Response> {
  Ok(Html(Wrapper(RepositoryWrapper(path, actual, render::Commits(CommitTree::new(repo))))).into())
}

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    render(context.requested_path.to_str().unwrap(), context.canonical_path.to_str().unwrap(), &context.repository)
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
