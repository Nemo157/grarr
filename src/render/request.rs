use git_appraise::{ Oid, Request };
use maud_pulldown_cmark::markdown;
use chrono::naive::datetime::NaiveDateTime;

fn summary(request: &Request) -> Option<&str> {
  request.description()
    .and_then(|description| description.lines().nth(0))
}

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: &Oid) -> String {
  oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

renderers! {
  RequestRenderer(id: &'a Oid, request: &'a Request) {
    div class="block request" {
      #RequestHeaderRenderer(id, request)
      #RequestDetailsRenderer(request)
    }
  }

  RequestStubRenderer(id: &'a Oid, request: &'a Request) {
    div class="request-stub" {
      a href={ "/" #id } {
        span class="id"
          #short(id)
        " "
        #if let Some(summary) = summary(request) {
          #summary
        }
        #if let None = summary(request) {
          "<No summary provided>"
        }
      }
    }
  }

  RequestHeaderRenderer(id: &'a Oid, request: &'a Request) {
    div class="block-header request-header" {
      h2 class="float-right" {
        a href={ "/" #id } {
          span class="id"
            #short(id)
        }
      }
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
          "<No summary provided>"
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
