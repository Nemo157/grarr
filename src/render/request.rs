use git_appraise::Request;
use maud_pulldown_cmark::markdown;
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  RequestRenderer(request: &'a Request) {
    div class="request" {
      #RequestHeaderRenderer(request)
      #RequestDetailsRenderer(request)
    }
  }

  RequestHeaderRenderer(request: &'a Request) {
    div class="request-header" {
      span {
        #if let Some(timestamp) = request.timestamp() {
          #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
          " "
        }
        "Merge from "
        code #request.review_ref().unwrap_or("<unknown ref>")
        " to "
        code #request.target_ref().unwrap_or("<unknown ref>")
        " requested by "
        #request.requester().unwrap_or("<unknown requester>")
      }
    }
  }

  RequestDetailsRenderer(request: &'a Request) {
    div class="request-details" {
      #if let Some(description) = request.description() {
        #(markdown::from_string(description))
      }
      #if let None = request.description() {
        i "No description provided"
      }
    }
  }
}
