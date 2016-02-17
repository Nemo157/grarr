use iron::headers::{ EntityTag, ETag, CacheControl, CacheDirective, Vary, IfNoneMatch };
use iron::modifier::Modifier;
use iron::modifiers::Header;
use iron::request::Request;
use iron::response::Response;
use iron::status;
use maud::RenderOnce;
use unicase::UniCase;

pub struct Html<'a, 'b: 'a, 'c: 'b, R: RenderOnce> {
  pub req: &'a Request<'b, 'c>,
  pub render: R,
  pub etag: Option<String>,
}

impl<'a, 'b, 'c, R: RenderOnce> Into<Response> for Html<'a, 'b, 'c, R> {
  fn into(self) -> Response {
    Response::with((status::Ok, self))
  }
}

impl<'a, 'b, 'c, R: RenderOnce> Modifier<Response> for Html<'a, 'b, 'c, R> {
  fn modify(self, response: &mut Response) {
    if let Some(etag) = self.etag {
      let entity_tag = EntityTag::weak(etag);
      let cache_headers = (
        Header(CacheControl(vec![
          CacheDirective::Public,
          CacheDirective::MaxAge(0),
        ])),
        Header(ETag(entity_tag.clone())),
        Header(Vary::Items(vec![
          UniCase("accept-encoding".to_owned()),
        ])),
      );
      cache_headers.modify(response);
      if let Some(&IfNoneMatch::Items(ref items)) = self.req.headers.get() {
        if items.len() == 1 && items[0] == entity_tag {
          status::NotModified.modify(response);
          return;
        }
      }
    }
    let buffer = to_string!(^self.render);
    let mime = mime!(Text/Html; Charset=Utf8);
    (mime, buffer).modify(response)
  }
}
