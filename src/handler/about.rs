use super::base::*;

#[derive(Clone)]
pub struct About;

impl Handler for About {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Html {
            render: render::About(),
            etag: None,
            req: req,
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

