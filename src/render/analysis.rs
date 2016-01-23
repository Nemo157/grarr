use git_appraise::{ Analysis };
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  AnalysisRenderer(analysis: &'a Analysis) {
    #if let Some(url) = analysis.url() {
      div class="block analysis" {
        div class="block-header" {
          div class="h3" {
            a href={ #url } {
              "External analysis"
              #if let Some(timestamp) = analysis.timestamp() {
                " submitted at "
                span class="timestamp" {
                  #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
                }
              }
            }
          }
        }
      }
    }
  }
}
