use router::Router;
use iron::middleware::Handler;

use super::route::Route;

pub trait Register {
  fn register<H: Route + Handler + Clone>(self, H) -> Self;
}

impl<'a> Register for &'a mut Router {
  fn register<H: Route + Handler + Clone>(self, handler: H) -> &'a mut Router{
    for route in <H as Route>::routes() {
      self.route(<H as Route>::method(), route, handler.clone());
    }
    self
  }
}
