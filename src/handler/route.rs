use std::borrow::Cow;
use hyper::method::Method;

pub trait Route {
  fn method() -> Method;
  fn route() -> Cow<'static, str> {
    "".into()
  }
  fn routes() -> Vec<Cow<'static, str>> {
    vec![Self::route()]
  }
}
