use iron::Error;
use super::{ ErrorRenderer };

renderers! {
  BadRequestRenderer(error: &'a Error) {
    h1 "Bad Request"
    h2 "Details"
    #ErrorRenderer(error)
  }
}
