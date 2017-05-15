use router::Router;
use iron::middleware::Handler;

use super::route::Route;

pub trait Register {
  fn register<H: Route + Handler + Clone>(self, H) -> Self;
}

impl<'a> Register for &'a mut Router {
  fn register<H: Route + Handler + Clone>(self, handler: H) -> &'a mut Router{
    for route in H::routes() {
      self.route(H::method(), route.clone(), handler.clone(), route);
    }
    self
  }
}
