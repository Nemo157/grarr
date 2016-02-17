use super::base::*;

use commit_tree::CommitTree;

#[derive(Clone)]
pub struct Commits;

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let commit = itry!(context.commit(), status::InternalServerError);
    let reference = context.reference().ok().and_then(|r| r.shorthand().map(ToOwned::to_owned)).unwrap_or_else(|| format!("{}", commit.id()));
    let commits = itry!(CommitTree::new(&context.repository, &commit), status::InternalServerError);
    Html {
      render: Wrapper(RepositoryWrapper(&context, render::Commits(&("/".to_owned() + context.requested_path.to_str().unwrap()), &reference, commits))),
      etag: Some(EntityTag::weak(versioned_sha1!(commit.id().as_bytes()))),
      req: req,
    }.into()
  }
}

impl Route for Commits {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/commits/:ref".into()
  }
}
