use git_appraise::{ self, Status };
use chrono::naive::datetime::NaiveDateTime;

pub fn CIStatus(ci_status: &git_appraise::CIStatus) -> ::maud::Markup {
    html! {
        div.block.ci-status {
            div.block-header {
                small {
                    @match ci_status.url() {
                        Some(url) => {
                            a href=(url) {
                                (CIStatusText(ci_status))
                            }
                        },
                        None => (CIStatusText(ci_status))
                    }
                }
            }
        }
    }
}

pub fn CIStatusText(ci_status: &git_appraise::CIStatus) -> ::maud::Markup {
    html! {
        span.agent {
            (ci_status.agent().unwrap_or("<Unknown agent>"))
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
            span.timestamp (NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
        }
    }
}
