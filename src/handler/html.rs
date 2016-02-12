use iron::modifier::Modifier;
use iron::response::Response;
use iron::status;
use maud::RenderOnce;

pub struct Html<R: RenderOnce>(pub R);

impl<R: RenderOnce> Into<Response> for Html<R> {
  fn into(self) -> Response {
    Response::with((status::Ok, self))
  }
}

impl<R: RenderOnce> Modifier<Response> for Html<R> {
  fn modify(self, response: &mut Response) {
    let buffer = to_string!(^self.0);
    let mime = mime!(Text/Html; Charset=Utf8);
    (mime, buffer).modify(response)
  }
}
