use iron;

pub fn Error(error: &iron::Error) -> ::maud::Markup {
  html! {
    pre.block-details code (error)
  }
}

pub fn BadRequest(error: &iron::Error) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h2 "Bad Request"
      }
      (Error(error))
    }
  }
}

pub fn NotFound(error: &iron::Error) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h2 "Not Found"
      }
      (Error(error))
    }
  }
}

pub fn InternalServerError(error: &iron::Error) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h2 "Internal Server Error"
      }
      (Error(error))
    }
  }
}
