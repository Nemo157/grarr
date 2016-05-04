use std::borrow::Cow;
use std::time::Duration;
use gravatar::{ self, Gravatar };
use hyper;
use hyper::client::Client;
use iron::IronResult;
use iron::headers::{ EntityTag, ContentType };
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;
use super::route::Route;
use router::Router;
use iron::status;
use iron::method::Method;
use lru_time_cache::LruCache;
use std::sync::Mutex;
use super::utils::{ self, sha1, File, CacheMatches };

pub struct Avatars {
  enable_gravatar: bool,
  cache: Option<Mutex<LruCache<String, File>>>,
  options: Options,
}

impl Clone for Avatars {
  fn clone(&self) -> Avatars {
    Avatars::new(self.options.clone())
  }
}

#[derive(Clone)]
pub struct Options {
  pub enable_gravatar: bool,
  pub enable_cache: bool,
  pub cache_capacity: usize,
  pub cache_time_to_live: Duration,
}

impl Avatars {
  pub fn new(options: Options) -> Avatars {
    Avatars {
      enable_gravatar: options.enable_gravatar,
      cache: if options.enable_cache {
        Some(Mutex::new(LruCache::with_expiry_duration_and_capacity(options.cache_time_to_live, options.cache_capacity)))
      } else {
        None
      },
      options: options,
    }
  }

  fn find_image(&self, user: &str) -> File {
    self.find_cached(user)
      .unwrap_or_else(||
        self.cache(user,
          self.find_gravatar(user)
            .unwrap_or_else(||
              self.default())))
  }

  fn cache(&self, user: &str, image: File) -> File {
    if let Some(ref cache) = self.cache {
      cache.lock().unwrap().insert(user.to_owned(), image.clone());
    }
    image
  }

  fn find_cached(&self, user: &str) -> Option<File> {
    self.cache.as_ref().and_then(|cache| cache.lock().unwrap().get(&user.to_owned()).cloned())
  }

  fn find_gravatar(&self, user: &str) -> Option<File> {
    if self.enable_gravatar {
      use std::io::Read;
      let mut gravatar = Gravatar::new(user);
      gravatar.size = Some(30);
      gravatar.default = Some(gravatar::Default::Identicon);
      let client = Client::new();
      let mut res = client.get(&gravatar.image_url()).send().unwrap();
      if res.status == hyper::Ok {
        let mut buf = Vec::new();
        res.read_to_end(&mut buf).unwrap();
        let mime = res.headers.get::<ContentType>().unwrap().0.clone();
        let entity_tag = EntityTag::strong(sha1(&buf));
        return Some(File(mime, entity_tag, buf.into()));
      }
    }
    None
  }

  fn default(&self) -> File {
    file!("../static/images/default_avatar.png")
  }
}

impl Handler for Avatars {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let user = req.extensions.get::<Router>().unwrap().find("user").unwrap();
    let File(mime, entity_tag, buffer) = self.find_image(user);
    let cache_headers = utils::cache_headers_for(&entity_tag, Duration::from_secs(86400));
    if req.cache_matches(&entity_tag) {
      return Ok(Response::with((status::NotModified, cache_headers)));
    }
    Ok(Response::with((status::Ok, mime, cache_headers, buffer.as_ref())))
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
