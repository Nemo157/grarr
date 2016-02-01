use git_appraise::{ Status, CIStatus };
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  CIStatusRenderer(ci_status: &'a CIStatus) {
    .block.ci-status {
      .block-header {
        .h3 {
          #match ci_status.url() {
            Some(url) => {
              a href={ #url } {
                #CIStatusTextRenderer(ci_status)
              }
            },
            None => #CIStatusTextRenderer(ci_status)
          }
        }
      }
    }
  }

  CIStatusTextRenderer(ci_status: &'a CIStatus) {
    span.agent {
      #ci_status.agent().unwrap_or("<Unknown agent>")
    }
    " reported status "
    span class={
      "status "
      #match ci_status.status() {
        Some(Status::Success) => "success",
        Some(Status::Failure) => "failure",
        None => "running",
      }
    } {
      #match ci_status.status() {
        Some(Status::Success) => "success",
        Some(Status::Failure) => "failure",
        None => "running",
      }
    }
    #if let Some(timestamp) = ci_status.timestamp() {
      " at "
      span.timestamp #NaiveDateTime::from_timestamp(timestamp.seconds(), 0)
    }
  }
}
