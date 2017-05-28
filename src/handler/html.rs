use std::time::Duration;
use std::io;
use std::fmt;
use iron::headers::EntityTag;
use iron::modifier::Modifier;
use iron::request::Request;
use iron::response::Response;
use iron::{ status, IronResult };
use super::utils::{ self, CacheMatches };
use maud::Render;
use render::{wrapper, Wrapper};
use settings::Settings;
use iron::response::WriteBody;

pub struct Html<'a, 'b: 'a, 'c: 'b, R: Render> {
    pub req: &'a Request<'b, 'c>,
    pub render: R,
    pub etag: Option<EntityTag>,
}

pub struct Html2<'a, 'b: 'a, 'c: 'b, D: Send + 'static, E: fmt::Display + 'static, R: FnOnce(D) -> E + Send + 'static> {
    pub req: &'a Request<'b, 'c>,
    pub etag: Option<EntityTag>,
    pub data: D,
    pub renderer: R,
}

pub struct Html2Body<D: Send + 'static, E: fmt::Display + 'static, R: FnOnce(D) -> E + Send + 'static> {
    pub settings: Option<Settings>,
    pub data: Option<D>,
    pub renderer: Option<R>,
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

impl<'a, 'b: 'a, 'c: 'b, D: Send + 'static, E: fmt::Display + 'static, R: FnOnce(D) -> E + Send> Into<Response> for Html2<'a, 'b, 'c, D, E, R> {
    fn into(self) -> Response {
        Response::with((status::Ok, self))
    }
}

impl<'a, 'b: 'a, 'c: 'b, D: Send + 'static, E: fmt::Display + 'static, R: FnOnce(D) -> E + Send + 'static> Into<IronResult<Response>> for Html2<'a, 'b, 'c, D, E, R> {
    fn into(self) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self)))
    }
}

impl<'a, 'b: 'a, 'c: 'b, D: Send, E: fmt::Display + 'static, R: FnOnce(D) -> E + Send + 'static> Modifier<Response> for Html2<'a, 'b, 'c, D, E, R> {
    fn modify(self, response: &mut Response) {
        if let Some(ref etag) = self.etag {
            let cache_headers = utils::cache_headers_for(&etag, Duration::from_secs(0));
            cache_headers.modify(response);
            if self.req.cache_matches(etag) {
                status::NotModified.modify(response);
                return;
            }
        }
        let mime = mime!(Text/Html; Charset=Utf8);
        let settings = self.req.extensions.get::<Settings>().cloned().unwrap_or_default();
        (mime, Box::new(Html2Body {
            settings: Some(settings),
            data: Some(self.data),
            renderer: Some(self.renderer),
        }) as Box<WriteBody>).modify(response)
    }
}

impl<D: Send + 'static, E: fmt::Display + 'static, R: FnOnce(D) -> E + Send + 'static> WriteBody for Html2Body<D, E, R> {
    fn write_body(&mut self, body: &mut io::Write) -> io::Result<()> {
        let settings = self.settings.take().unwrap();
        let renderer = self.renderer.take().unwrap();
        let data = self.data.take().unwrap();
        let wrapped = renderer(data);
        write!(body, "{}", wrapper(settings, wrapped))
    }
}
