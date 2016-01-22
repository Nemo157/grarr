use git_appraise::{ Comment, Comments };
use maud_pulldown_cmark::markdown;
use chrono::naive::datetime::NaiveDateTime;

renderers! {
  CommentsRenderer(comments: Comments) {
    div class="comments" {
      #for comment in comments {
        #(CommentRenderer(comment))
      }
    }
  }

  CommentHeaderRendererer(comment: &'a Comment) {
    div class="block-header comment-header" {
      h3 {
        span class="email"
          #comment.author().unwrap_or("<unknown author>")
        " commented "
        #if let Some(timestamp) = comment.timestamp() {
          "on "
          span class="timestamp"
            #(NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
          " "
        }
        "with status "
        span class={
          "status "
          #comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi")
        } {
          #comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi")
          // #comment.resolved().map(|r| if r { "👍" } else { "👎" }).unwrap_or("ℹ️")
        }
      }
    }
  }

  CommentDetailsRendererer(comment: &'a Comment) {
    #if let Some(location) = comment.location() {
      div class="block-details comment-details" {
        pre { code { #(format!("{:?}", location)) } }
      }
    }
    #if let Some(description) = comment.description() {
      div class="block-details comment-details" {
        #(markdown::from_string(description))
      }
    }
  }

  CommentRenderer(comment: Comment) {
    div class="block comment" {
      #CommentHeaderRendererer(&comment)
      #CommentDetailsRendererer(&comment)
    }
  }
}
