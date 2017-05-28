use super::base::*;
use super::html::Html2;

#[derive(Clone)]
pub struct About;

impl Handler for About {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Html2 {
            req: req,
            etag: None,
            renderer: render::about,
            data: (),
        }.into()
    }
}

impl Route for About {
    fn method() -> Method {
        Method::Get
    }

    fn route() -> Cow<'static, str> {
        "/-/about".into()
    }
}

