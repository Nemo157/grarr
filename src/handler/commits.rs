use super::base::*;

use commit_tree::CommitTree;

#[derive(Clone)]
pub struct Commits;

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let reff = itry!(router.find("ref").ok_or(Error::MissingPathComponent), status::InternalServerError);
    let object = itry!(context.repository.revparse_single(reff), status::NotFound);
    let commit = itry!(object.as_commit().ok_or(Error::String("Object is not commit...")), status::InternalServerError);
    let commits = itry!(CommitTree::new(&context.repository, &commit), status::InternalServerError);
    Html {
      render: Wrapper(RepositoryWrapper(&context, render::Commits(&("/".to_owned() + context.requested_path.to_str().unwrap()), &reff, commits))),
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
