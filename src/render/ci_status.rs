use git_appraise::{ Status, CIStatus };
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  CIStatusRenderer(ci_status: &'a CIStatus) {
    div class="block ci-status" {
      div class="block-header" {
        h3 {
          #if let Some(url) = ci_status.url() {
            a href={ #url } {
              #CIStatusTextRenderer(ci_status)
            }
          }
          #if let None = ci_status.url() {
            #CIStatusTextRenderer(ci_status)
          }
        }
      }
    }
  }

  CIStatusTextRenderer(ci_status: &'a CIStatus) {
    span class="agent" {
      #ci_status.agent().unwrap_or("<Unknown agent>")
    }
    " reported status "
    span class={
      "status "
      #ci_status.status().map(|s| match s {
        Status::Success => "success",
        Status::Failure => "failure"
      }).unwrap_or("running")
    } {
      #ci_status.status().map(|s| match s {
        Status::Success => "success",
        Status::Failure => "failure"
      }).unwrap_or("running")
    }
    #if let Some(timestamp) = ci_status.timestamp() {
      " at "
      span class="timestamp"
        #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
    }
  }
}
