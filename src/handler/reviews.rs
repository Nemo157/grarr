use super::base::*;
use git_appraise::AppraisedRepository;

#[derive(Clone)]
pub struct Reviews;

impl Handler for Reviews {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
        let mut reviews: Vec<_> = context.repository.all_reviews().and_then(|revs| revs.collect()).ok().unwrap_or_default();
        reviews.sort_by(|a, b| a.request().timestamp().cmp(&b.request().timestamp()));
        reviews.reverse();
        let root = format!("/{}", context.path);
        Html {
            render: RepositoryWrapper(&context, render::Reviews(&root, &reviews), Some(render::Tab::Reviews)),
            etag: None,
            req: req,
        }.into()
    }
}

impl Route for Reviews {
    fn method() -> Method {
        Method::Get
    }

    fn route() -> Cow<'static, str> {
        "/reviews".into()
    }
}
