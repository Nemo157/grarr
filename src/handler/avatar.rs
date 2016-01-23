use gravatar::{ self, Gravatar };
use hyper;
use hyper::client::Client;
use iron::IronResult;
use iron::headers::ContentType;
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;
use mime::Mime;
use super::route::Route;
use router::Router;
use iron::status;
use iron::method::Method;

pub struct Avatars {
  pub enable_gravatar: bool,
}

struct Image(Mime, Vec<u8>);

impl Avatars {
  fn find_image(&self, user: &str) -> Image {
    self.find_gravatar(user)
      .unwrap_or_else(|| self.default())
  }

  fn find_gravatar(&self, user: &str) -> Option<Image> {
    if self.enable_gravatar {
      use std::io::Read;
      let mut gravatar = Gravatar::new(user);
      gravatar.size = Some(30);
      gravatar.default = Some(gravatar::Default::Identicon);
      let client = Client::new();
      let mut res = client.get(&gravatar.image_url()).send().unwrap();
      assert_eq!(res.status, hyper::Ok);
      let mut buf = Vec::new();
      res.read_to_end(&mut buf).unwrap();
      let mime = res.headers.get::<ContentType>().unwrap().0.clone();
      Some(Image(mime, buf))
    } else {
      None
    }
  }

  fn default(&self) -> Image {
    unimplemented!()
  }
}

impl Handler for Avatars {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let user = req.extensions.get::<Router>().unwrap().find("user").unwrap();
    let Image(mime, buffer) = self.find_image(user);
    Ok(Response::with((status::Ok, mime, buffer)))
  }
}

impl Route for Avatars {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> &'static str {
    "/avatars/:user"
  }
}
