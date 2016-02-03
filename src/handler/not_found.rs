use super::base::*;

use iron::middleware::AfterMiddleware;

pub struct NotFound;

impl AfterMiddleware for NotFound {
  fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
    if err.response.status == Some(status::NotFound) {
      Ok(Response::with((status::NotFound, Html(&Wrapper(render::NotFound(&*err.error))))))
    } else {
      Err(err)
    }
  }
}
