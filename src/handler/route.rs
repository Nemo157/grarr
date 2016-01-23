use hyper::method::Method;

pub trait Route {
  fn route() -> &'static str;
  fn method() -> Method;
}
