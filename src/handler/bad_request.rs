use super::base::*;

use iron::middleware::AfterMiddleware;

use render::BadRequestRenderer;

pub struct BadRequest;

impl AfterMiddleware for BadRequest {
  fn after(&self, _: &mut Request, response: Response) -> IronResult<Response> {
    if response.status == Some(status::BadRequest) {
      println!("{:?}", response);
      Ok(response)
    } else {
      Ok(response)
    }
  }

  fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
    if err.response.status == Some(status::BadRequest) {
      Ok(Response::with((status::BadRequest, Html(&Wrapper(BadRequestRenderer(&*err.error))))))
    } else {
      Err(err)
    }
  }
}
