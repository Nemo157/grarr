use iron;

renderers! {
  Error(error: &'a iron::Error) {
    pre.block-details code ^error
  }

  BadRequest(error: &'a iron::Error) {
    div.block {
      div.block-header {
        h2 "Bad Request"
      }
      ^Error(error)
    }
  }

  NotFound(error: &'a iron::Error) {
    div.block {
      div.block-header {
        h2 "Not Found"
      }
      ^Error(error)
    }
  }

  InternalServerError(error: &'a iron::Error) {
    div.block {
      div.block-header {
        h2 "Internal Server Error"
      }
      ^Error(error)
    }
  }
}
