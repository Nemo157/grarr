use super::base::*;

use iron::middleware::AfterMiddleware;

macro_rules! error_handler {
  ($name:ident) => {
    pub struct $name;
    impl AfterMiddleware for $name {
      fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        if err.response.status == Some(status::$name) {
          Ok(Response::with((status::$name, Html {
            render: &render::error::$name(&*err.error),
            etag: None,
            req: req,
          })))
        } else {
          Err(err)
        }
      }
    }
  }
}

error_handler!(BadRequest);
error_handler!(NotFound);
error_handler!(InternalServerError);
