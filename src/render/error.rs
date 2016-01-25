use iron::Error;

renderers! {
  ErrorRenderer(error: &'a Error) {
    pre code #error
  }
}
