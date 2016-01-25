use iron::Error;
use super::{ ErrorRenderer };

renderers! {
  NotFoundRenderer(error: &'a Error) {
    h1 "Not Found"
    h2 "Details"
    #ErrorRenderer(error)
  }
}
