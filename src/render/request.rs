use git_appraise::{ Oid, Request };
use maud_pulldown_cmark::markdown;
use chrono::naive::datetime::NaiveDateTime;
use super::{ Avatar };

fn summary(request: &Request) -> Option<&str> {
  request.description()
    .and_then(|description| description.lines().nth(0))
}

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: Oid) -> String {
  oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

renderers! {
  RequestRenderer(request: &'a Request) {
    .block.request {
      #RequestHeaderRenderer(request)
      #RequestDetailsRenderer(request)
    }
  }

  RequestStubRenderer(request: &'a Request) {
    .request-stub {
      a href={ "reviews/" #request.commit_id() } {
        span.id
          #short(request.commit_id())
        " "
        #match summary(request) {
          Some(summary) => #summary,
          None => "<No summary provided>",
        }
      }
    }
  }

  RequestHeaderRenderer(request: &'a Request) {
    .block-header.request-header {
      h2.float-right {
        a href={ #request.commit_id() } {
          span.id
            #short(request.commit_id())
        }
      }
      .h4 {
        #if let Some(timestamp) = request.timestamp() {
          span.timestamp
            #NaiveDateTime::from_timestamp(timestamp.seconds(), 0)
        }
      }
      .h2 {
        #match summary(request) {
          Some(summary) => #summary,
          None => "<No summary provided>",
        }
      }
      .h3 {
        #Avatar(request.requester().unwrap_or("unknown@example.org"))
        span.rest {
          span.user
            #request.requester().unwrap_or("<unknown requester>")
          " wants to merge "
          span.ref
            #request.review_ref().unwrap_or("<unknown ref>")
          " into "
          span.ref
            #request.target_ref().unwrap_or("<unknown ref>")
        }
      }
    }
  }

  RequestDetailsRenderer(request: &'a Request) {
    .block-details.request-details {
      #match request.description() {
        Some(description) => #markdown::from_string(description),
        None => i "No description provided",
      }
    }
  }
}
