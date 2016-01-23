use iron::IronResult;
use iron::middleware::Handler;
use iron::request::Request;
use iron::response::Response;

struct Router {
  router: ::router::Router,
}

impl Router {
  pub fn new() {
    Router {
      router: create_router(),
    }
  }

  fn create_router() -> ::router::Router {
    let mut router = Router::new();
    router.get("/", reviews_handler);
    router.get("/avatars/:user", avatars_handler);
    router.get("/:commit_id", review_handler);
    router
  }
}

impl Handler for Router {
  fn handle(&self, request: &mut Request) -> IronResult<Response> {
    self.router.handle(request)
  }
}
