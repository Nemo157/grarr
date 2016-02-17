use std::borrow::Cow;
use iron::IronResult;
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;
use super::route::Route;
use router::Router;
use iron::status;
use iron::method::Method;
use std::sync::Mutex;
use error::Error;
use std::collections::HashMap;
use std::path::{ Path, PathBuf };
use iron::modifiers::Header;
use iron::headers::{ ETag, CacheControl, CacheDirective, Vary, IfNoneMatch };
use unicase::UniCase;
use super::utils::File;

#[derive(Debug)]
pub struct Static {
  files: Mutex<HashMap<PathBuf, File>>,
}

impl Clone for Static {
  fn clone(&self) -> Static {
    Static::new(self.files.lock().unwrap().clone())
  }
}

#[macro_export]
macro_rules! statics {
  (prefix: $prefix:expr; $($x:expr),*) => (
    $crate::handler::statics::Static::new(vec![
      $(($x.trim_left_matches($prefix).into(), file!($x))),*
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
    let File(mime, entity_tag, buffer) = itry!(self.find_file(path).ok_or(Error::String("Static file not found")), status::NotFound);
    let cache_headers = (
      Header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(86400),
      ])),
      Header(ETag(entity_tag.clone())),
      Header(Vary::Items(vec![
        UniCase("accept-encoding".to_owned()),
      ])),
    );
    if let Some(&IfNoneMatch::Items(ref items)) = req.headers.get() {
      if items.len() == 1 && items[0] == entity_tag {
        return Ok(Response::with((status::NotModified, cache_headers)));
      }
    }
    Ok(Response::with((status::Ok, mime, cache_headers, &*buffer)))
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
