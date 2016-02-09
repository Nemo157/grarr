use std::borrow::Cow;
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
use lru_time_cache::LruCache;
use time::Duration;
use std::sync::Mutex;

pub struct Avatars {
  enable_gravatar: bool,
  cache: Option<Mutex<LruCache<String, Image>>>,
}

pub struct Options {
  pub enable_gravatar: bool,
  pub enable_cache: bool,
  pub cache_capacity: usize,
  pub cache_time_to_live: Duration,
}

#[derive(Clone)]
struct Image(Mime, Vec<u8>);

impl Avatars {
  pub fn new(options: Options) -> Avatars {
    Avatars {
      enable_gravatar: options.enable_gravatar,
      cache: match options.enable_cache {
        false => None,
        true => Some(Mutex::new(LruCache::with_expiry_duration_and_capacity(options.cache_time_to_live, options.cache_capacity))),
      }
    }
  }

  fn find_image(&self, user: &str) -> Image {
    self.find_cached(user)
      .unwrap_or_else(||
        self.cache(user,
          self.find_gravatar(user)
            .unwrap_or_else(||
              self.default())))
  }

  fn cache(&self, user: &str, image: Image) -> Image {
    if let Some(ref cache) = self.cache {
      cache.lock().unwrap().insert(user.to_owned(), image.clone());
    }
    image
  }

  fn find_cached(&self, user: &str) -> Option<Image> {
    self.cache.as_ref().and_then(|cache| cache.lock().unwrap().get(&user.to_owned()).cloned())
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

  fn route() -> Cow<'static, str> {
    "/-/avatars/:user".into()
  }
}
