use std::borrow::Cow;
use std::fmt;
use iron::IronResult;
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;
use mime::Mime;
use super::route::Route;
use router::Router;
use iron::status;
use iron::method::Method;
use std::sync::Mutex;
use error::Error;
use std::collections::HashMap;
use std::path::{ Path, PathBuf };
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use iron::modifiers::Header;
use iron::headers;
use unicase::UniCase;

#[derive(Debug)]
pub struct Static {
  files: Mutex<HashMap<PathBuf, File>>,
}

#[derive(Clone, Debug)]
pub struct Sha1Hash(pub String);

#[derive(Clone)]
pub struct File(pub Mime, pub Sha1Hash, pub &'static [u8]);

impl fmt::Debug for File {
  fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
    write!(w, "File{:?}", (&self.0, &self.1, &self.2.len()))
  }
}

pub fn mime(path: &str) -> Mime {
  match Path::new(path).extension().and_then(|s| s.to_str()) {
    Some("css") => mime!(Text/Css),
    Some("js") => mime!(Text/Javascript),
    None | Some(_) => mime!(Application/("octet-stream")),
  }
}

pub fn sha1(file: &[u8]) -> Sha1Hash {
  let mut hasher = Sha1::new();
  hasher.input(file);
  Sha1Hash(hasher.result_str())
}

#[macro_export]
macro_rules! statics {
  (prefix: $prefix:expr; $($x:expr),*) => (
    $crate::handler::statics::Static::new(vec![
      $((
        $x.trim_left_matches($prefix).into(),
        $crate::handler::statics::File($crate::handler::statics::mime($x), $crate::handler::statics::sha1(include_bytes!($x)), include_bytes!($x))
      )),*
    ].into_iter().collect())
  );
  (prefix: $prefix:expr; $($x:expr,)*) => (statics![prefix: $prefix; $($x),*]);
}

impl Static {
  pub fn new(files: HashMap<PathBuf, File>) -> Static {
    Static {
      files: Mutex::new(files),
    }
  }

  fn find_file(&self, path: &Path) -> Option<File> {
    self.files.lock().unwrap().get(path).cloned()
  }
}

impl Handler for Static {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
    let path = Path::new(itry!(router.find("path").ok_or(Error::MissingPathComponent), status::InternalServerError));
    let File(mime, Sha1Hash(sha), buffer) = itry!(self.find_file(path).ok_or(Error::FromString("Static file not found")), status::NotFound);
    let entity_tag = headers::EntityTag::strong(sha);
    let cache_headers = (
      Header(headers::CacheControl(vec![
        headers::CacheDirective::Public,
        headers::CacheDirective::MaxAge(86400),
      ])),
      Header(headers::ETag(entity_tag.clone())),
      Header(headers::Vary::Items(vec![
        UniCase("accept-encoding".to_owned()),
      ])),
    );
    if let Some(&headers::IfNoneMatch::Items(ref items)) = req.headers.get() {
      if items.len() == 1 && items[0] == entity_tag {
        return Ok(Response::with((status::NotModified, cache_headers)));
      }
    }
    Ok(Response::with((status::Ok, mime, cache_headers, buffer)))
  }
}

impl Route for Static {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/-/static/*path".into()
  }
}
