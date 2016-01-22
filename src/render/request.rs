use git_appraise::Request;
use maud_pulldown_cmark::markdown;
use chrono::naive::datetime::NaiveDateTime;

fn summary(request: &Request) -> Option<&str> {
  request.description()
    .and_then(|description| description.lines().nth(0))
}

renderers! {
  RequestRenderer(request: &'a Request) {
    div class="block request" {
      #RequestHeaderRenderer(request)
      #RequestDetailsRenderer(request)
    }
  }

  RequestHeaderRenderer(request: &'a Request) {
    div class="block-header request-header" {
      h4 {
        #if let Some(timestamp) = request.timestamp() {
          span class="timestamp"
            #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
        }
      }
      h2 {
        #if let Some(summary) = summary(request) {
          #summary
        }
        #if let None = summary(request) {
          "No summary provided"
        }
      }
      h3 {
        span class="email"
          #request.requester().unwrap_or("<unknown requester>")
        " wants to merge "
        code class="ref"
          #request.review_ref().unwrap_or("<unknown ref>")
        " into "
        code class="ref"
          #request.target_ref().unwrap_or("<unknown ref>")
      }
    }
  }

  RequestDetailsRenderer(request: &'a Request) {
    div class="block-details request-details" {
      #if let Some(description) = request.description() {
        #(markdown::from_string(description))
      }
      #if let None = request.description() {
        i "No description provided"
      }
    }
  }
}
