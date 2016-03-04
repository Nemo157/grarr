use super::base::*;

use commit_tree::CommitTree;

#[derive(Clone)]
pub struct Commits;

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let referenced_commit = itry!(context.referenced_commit(), status::InternalServerError);
    let commits = itry!(CommitTree::new(&context.repository, &referenced_commit.commit), status::InternalServerError);
    Html {
      render: RepositoryWrapper(&context, render::Commits(&("/".to_owned() + context.requested_path.to_str().unwrap()), &referenced_commit, commits)),
      etag: Some(EntityTag::weak(versioned_sha1!(referenced_commit.commit.id().as_bytes()))),
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
