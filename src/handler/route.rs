use std::borrow::Cow;
use hyper::method::Method;

pub trait Route {
  fn route() -> Cow<'static, str>;
  fn method() -> Method;
}
