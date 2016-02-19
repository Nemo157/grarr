use std::time::Duration;
use iron::headers::EntityTag;
use iron::modifier::Modifier;
use iron::request::Request;
use iron::response::Response;
use iron::{ status, IronResult };
use maud::RenderOnce;
use super::utils::{ self, CacheMatches };

pub struct Html<'a, 'b: 'a, 'c: 'b, R: RenderOnce> {
  pub req: &'a Request<'b, 'c>,
  pub render: R,
  pub etag: Option<EntityTag>,
}

impl<'a, 'b, 'c, R: RenderOnce> Into<Response> for Html<'a, 'b, 'c, R> {
  fn into(self) -> Response {
    Response::with((status::Ok, self))
  }
}

impl<'a, 'b, 'c, R: RenderOnce> Into<IronResult<Response>> for Html<'a, 'b, 'c, R> {
  fn into(self) -> IronResult<Response> {
    Ok(Response::with((status::Ok, self)))
  }
}

impl<'a, 'b, 'c, R: RenderOnce> Modifier<Response> for Html<'a, 'b, 'c, R> {
  fn modify(self, response: &mut Response) {
    if let Some(ref etag) = self.etag {
      let cache_headers = utils::cache_headers_for(&etag, Duration::from_secs(0));
      cache_headers.modify(response);
      if self.req.cache_matches(etag) {
        status::NotModified.modify(response);
        return;
      }
    }
    let buffer = to_string!(^self.render);
    let mime = mime!(Text/Html; Charset=Utf8);
    (mime, buffer).modify(response)
  }
}
