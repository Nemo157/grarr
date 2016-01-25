use iron::response::Response;
use iron::status;
use maud::RenderOnce;
use mime::Mime;

pub struct Html<R: RenderOnce>(pub R);

impl<R: RenderOnce> Into<Response> for Html<R> {
  fn into(self) -> Response {
    let buffer = to_string!(#(self.0));
    let mime: Mime = "text/html".parse().unwrap();
    Response::with((status::Ok, mime, buffer))
  }
}
