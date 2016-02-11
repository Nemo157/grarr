use git_appraise::{ self, Status };
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  CIStatus(ci_status: &'a git_appraise::CIStatus) {
    div.block.ci-status {
      div.block-header {
        div.h3 {
          @match ci_status.url() {
            Some(url) => {
              a href={ ^url } {
                ^CIStatusText(ci_status)
              }
            },
            None => ^CIStatusText(ci_status)
          }
        }
      }
    }
  }

  CIStatusText(ci_status: &'a git_appraise::CIStatus) {
    span.agent {
      ^ci_status.agent().unwrap_or("<Unknown agent>")
    }
    " reported status "
    span class={
      "status "
      @match ci_status.status() {
        Some(Status::Success) => "success",
        Some(Status::Failure) => "failure",
        None => "running",
      }
    } {
      @match ci_status.status() {
        Some(Status::Success) => "success",
        Some(Status::Failure) => "failure",
        None => "running",
      }
    }
    @if let Some(timestamp) = ci_status.timestamp() {
      " at "
      span.timestamp ^NaiveDateTime::from_timestamp(timestamp.seconds(), 0)
    }
  }
}
