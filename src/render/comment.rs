use git_appraise::{ Comment };
use maud_pulldown_cmark::markdown;
use super::{ Avatar };
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  CommentHeaderRendererer(comment: &'a Comment) {
    .block-header.comment-header {
      .h3 {
        #Avatar(comment.author().unwrap_or("unknown@example.org"))
        span.rest {
          span.user
            #comment.author().unwrap_or("<unknown author>")
          " commented "
          #if let Some(timestamp) = comment.timestamp() {
            "on "
            span.timestamp
              #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
            " "
          }
          "with status "
          span class={
            "resolved "
            #comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi")
          } {
            #comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi")
            // #comment.resolved().map(|r| if r { "👍" } else { "👎" }).unwrap_or("ℹ️")
          }
        }
      }
    }
  }

  CommentDetailsRendererer(comment: &'a Comment) {
    #if let Some(location) = comment.location() {
      .block-details.comment-details {
        pre { code { #(format!("{:?}", location)) } }
      }
    }
    #if let Some(description) = comment.description() {
      .block-details.comment-details {
        #(markdown::from_string(description))
      }
    }
  }

  CommentRenderer(comment: &'a Comment) {
    .block.comment {
      #CommentHeaderRendererer(comment)
      #CommentDetailsRendererer(comment)
    }
  }
}
