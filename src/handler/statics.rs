use std::borrow::Cow;
use std::time::Duration;
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
use super::utils::{ self, File, CacheMatches };

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
        let router = itry!(req.extensions.get::<Router>().ok_or(Error::from("missing extension")), status::InternalServerError);
        let path = Path::new(itry!(router.find("path").ok_or(Error::from("missing path component")), status::InternalServerError));
        let File(mime, entity_tag, buffer) = itry!(self.find_file(path).ok_or(Error::from("Static file not found")), status::NotFound);
        let cache_headers = utils::cache_headers_for(&entity_tag, Duration::from_secs(86400));
        if req.cache_matches(&entity_tag) {
            return Ok(Response::with((status::NotModified, cache_headers)));
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
