use iron::middleware::AfterMiddleware;
use iron::request::Request;
use iron::response::Response;
use iron::{self, IronResult, IronError};

use render;
use super::html::Html2;

pub struct Error;
impl AfterMiddleware for Error {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(status) = err.response.status {
            if status.is_client_error() || status.is_server_error() {
                Ok(Response::with((status, Html2 {
                    req: req,
                    etag: None,
                    renderer: render::error,
                    data: (status, err.error),
                })))
            } else {
                Err(err)
            }
        } else {
            Ok(Response::with((iron::status::InternalServerError, Html2 {
                req: req,
                etag: None,
                renderer: render::error,
                data: (iron::status::InternalServerError, err.error),
            })))
        }
    }
}
