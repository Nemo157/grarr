use router::Router;
use iron::middleware::Handler;

use super::route::Route;

pub trait Register {
  fn register<H: Route + Handler>(self, H) -> Self;
}

impl<'a> Register for &'a mut Router {
  fn register<H: Route + Handler>(self, handler: H) -> &'a mut Router{
    self.route(<H as Route>::method(), <H as Route>::route(), handler)
  }
}
