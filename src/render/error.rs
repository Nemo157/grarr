use iron;

renderers! {
  Error(error: &'a iron::Error) {
    pre code ^error
  }

  BadRequest(error: &'a iron::Error) {
    h1 "Bad Request"
    h2 "Details"
    ^Error(error)
  }

  NotFound(error: &'a iron::Error) {
    h1 "Not Found"
    h2 "Details"
    ^Error(error)
  }

  InternalServerError(error: &'a iron::Error) {
    h1 "Internal Server Error"
    h2 "Details"
    ^Error(error)
  }
}
