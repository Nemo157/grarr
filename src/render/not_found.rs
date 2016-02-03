use iron::Error;

renderers! {
  NotFound(error: &'a Error) {
    h1 "Not Found"
    h2 "Details"
    ^super::Error(error)
  }
}
