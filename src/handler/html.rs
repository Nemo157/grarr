use std::time::Duration;
use iron::headers::EntityTag;
use iron::modifier::Modifier;
use iron::request::Request;
use iron::response::Response;
use iron::{ status, IronResult };
use maud::Render;
use super::utils::{ self, CacheMatches };
use render::Wrapper;
use settings::Settings;

pub struct Html<'a, 'b: 'a, 'c: 'b, R: Render> {
    pub req: &'a Request<'b, 'c>,
    pub render: R,
    pub etag: Option<EntityTag>,
}

impl<'a, 'b, 'c, R: Render> Into<Response> for Html<'a, 'b, 'c, R> {
    fn into(self) -> Response {
        Response::with((status::Ok, self))
    }
}

impl<'a, 'b, 'c, R: Render> Into<IronResult<Response>> for Html<'a, 'b, 'c, R> {
    fn into(self) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self)))
    }
}

impl<'a, 'b, 'c, R: Render> Modifier<Response> for Html<'a, 'b, 'c, R> {
    fn modify(self, response: &mut Response) {
        if let Some(ref etag) = self.etag {
            let cache_headers = utils::cache_headers_for(&etag, Duration::from_secs(0));
            cache_headers.modify(response);
            if self.req.cache_matches(etag) {
                status::NotModified.modify(response);
                return;
            }
        }
        let settings = self.req.extensions.get::<Settings>().cloned().unwrap_or_default();
        let buffer = Wrapper(self.render, settings).render();
        let mime = mime!(Text/Html; Charset=Utf8);
        (mime, buffer).modify(response)
    }
}
