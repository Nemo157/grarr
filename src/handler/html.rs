use iron::modifier::Modifier;
use iron::response::Response;
use iron::status;
use maud::RenderOnce;
use mime::Mime;

pub struct Html<R: RenderOnce>(pub R);

impl<R: RenderOnce> Into<Response> for Html<R> {
  fn into(self) -> Response {
    Response::with((status::Ok, self))
  }
}

impl<R: RenderOnce> Modifier<Response> for Html<R> {
  fn modify(self, response: &mut Response) {
    let buffer = to_string!(#(self.0));
    let mime: Mime = "text/html; charset=UTF-8".parse().unwrap();
    (mime, buffer).modify(response)
  }
}
