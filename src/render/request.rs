use git_appraise::Request;
use maud_pulldown_cmark::markdown;
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  RequestRenderer(request: &'a Request) {
    div {
      #if let Some(requester) = request.requester() {
        div { "Requester: " #requester }
      }
      #if let Some(timestamp) = request.timestamp() {
        div { "Timestamp: " #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0)) }
      }
      #if let (Some(review_ref), Some(target_ref)) = (request.review_ref(), request.target_ref()) {
        div { "Proposed merge: " #review_ref " -> " #target_ref }
      }
      #if let Some(reviewers) = request.reviewers() {
        div { "Reviewers:"
          ul {
            #for reviewer in reviewers {
              li #reviewer
            }
          }
        }
      }
      #if let Some(ref description) = request.description() {
        div {
          "Description: "
          div { #(markdown::from_string(description)) }
        }
      }
    }
  }
}
