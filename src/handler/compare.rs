use super::base::*;

use referenced_commit::ReferencedCommit;

#[derive(Clone)]
pub struct Compare;

impl Handler for Compare {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
        let new_commit = itry!(context.referenced_commit(), status::NotFound);

        // TODO: Read which old commit from url
        let old_commit = itry!(
            context.repository
                .head().map_err(Error::from)
                .and_then(|h|
                    h.resolve().map_err(Error::from)
                        .and_then(|h| h.target().ok_or(Error::from("<.<    >.>    <.<")))
                        .and_then(|id| context.repository.find_commit(id).map_err(Error::from))
                        .map(|commit| ReferencedCommit { commit: commit, reference: Some(h) })),
            status::InternalServerError);

        let base = itry!(
            context.repository
                .merge_base(old_commit.commit.id(), new_commit.commit.id())
                .and_then(|id| context.repository.find_commit(id)),
            status::InternalServerError);

        let commits = itry!(
            context.repository
                .revwalk()
                .and_then(|mut walker| walker.push(new_commit.commit.id()).map(|_| walker))
                .and_then(|mut walker| walker.hide(base.id()).map(|_| walker))
                .and_then(|walker|
                    walker.map(|id| id.and_then(|id| context.repository.find_commit(id)))
                        .collect()),
            status::InternalServerError);

        Html {
            render: RepositoryWrapper(&context, render::Compare {
                context: &context,
                new: new_commit,
                old: old_commit,
                base: base,
                commits: commits,
            }, None),
            etag: Some(EntityTag::weak(versioned_sha1!())),
            req: req,
        }.into()
    }
}

impl Route for Compare {
    fn method() -> Method {
        Method::Get
    }

    fn routes() -> Vec<Cow<'static, str>> {
        vec![
            "/compare/:ref".into(),
            // "/compare/:new_ref...:old_ref".into(),
        ]
    }
}
