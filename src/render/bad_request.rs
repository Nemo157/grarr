use iron::Error;

renderers! {
  BadRequest(error: &'a Error) {
    h1 "Bad Request"
    h2 "Details"
    ^super::Error(error)
  }
}
