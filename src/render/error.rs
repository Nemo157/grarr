use iron;

renderers! {
  Error(error: &'a iron::Error) {
    pre code ^error
  }
}
