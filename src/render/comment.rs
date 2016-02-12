use git_appraise;
use maud_pulldown_cmark::Markdown;
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  CommentHeader(comment: &'a git_appraise::Comment) {
    div.block-header.comment-header {
      h4 {
        ^super::Avatar(comment.author().unwrap_or("unknown@example.org"), &None)
        span.rest {
          span.user
            ^comment.author().unwrap_or("<unknown author>")
          " commented "
          @if let Some(timestamp) = comment.timestamp() {
            "on "
            span.timestamp
              ^NaiveDateTime::from_timestamp(timestamp.seconds(), 0)
            " "
          }
          "with status "
          span class={
            "resolved "
            @match comment.resolved() {
              Some(true) => "lgtm",
              Some(false) => "nmw",
              None => "fyi",
            }
          } {
            @match comment.resolved() {
              Some(true) => "👍",
              Some(false) => "👎",
              None => "ℹ️",
            }
          }
        }
      }
    }
  }

  CommentDetails(comment: &'a git_appraise::Comment) {
    @if let Some(location) = comment.location() {
      div.block-details.comment-details {
        pre { code { ^(format!("{:?}", location)) } }
      }
    }
    @if let Some(description) = comment.description() {
      div.block-details.comment-details {
        ^Markdown::from_string(description)
      }
    }
  }

  Comment(comment: &'a git_appraise::Comment) {
    div.block.comment {
      ^CommentHeader(comment)
      ^CommentDetails(comment)
    }
  }
}
