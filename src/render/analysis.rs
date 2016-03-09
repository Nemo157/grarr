use git_appraise;
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  Analysis(analysis: &'a git_appraise::Analysis) {
    @if let Some(url) = analysis.url() {
      div.block.analysis {
        div.block-header {
          small {
            a href={ ^url } {
              "External analysis"
              @if let Some(timestamp) = analysis.timestamp() {
                " submitted at "
                span.timestamp {
                  ^NaiveDateTime::from_timestamp(timestamp.seconds(), 0)
                }
              }
            }
          }
        }
      }
    }
  }
}
