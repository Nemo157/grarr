use std::borrow::Cow;
use iron::method::Method;

pub trait Route {
  fn method() -> Method;

  fn route() -> Cow<'static, str> {
    "".into()
  }

  fn routes() -> Vec<Cow<'static, str>> {
    vec![Self::route()]
  }
}
