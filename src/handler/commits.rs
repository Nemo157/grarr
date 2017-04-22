use super::base::*;

use git2::Oid;
use commit_tree::CommitTree;

#[derive(Clone)]
pub struct Commits;

impl Handler for Commits {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let start_commit = req.url.clone().into_generic_url()
      .query_pairs()
      // .unwrap_or_default()
      .into_iter()
      .find(|&(ref key, _)| key == "start")
      .map(|(_, ref id)| Oid::from_str(id)
        .and_then(|id| context.repository.find_commit(id)));
    let referenced_commit = itry!(context.referenced_commit(), status::InternalServerError);
    let initial_commit = if let Some(Ok(ref commit)) = start_commit {
      commit
    } else {
      &referenced_commit.commit
    };
    let commits = itry!(CommitTree::new(&context.repository, &initial_commit, 50), status::InternalServerError);
    Html {
      render: RepositoryWrapper(&context, render::Commits(&context, &referenced_commit, commits), Some(render::Tab::Commits)),
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
