use git_appraise;
use maud_pulldown_cmark::markdown;

renderers! {
  CommentRenderer(comment: &'a git_appraise::Comment) {
    div {
      #if let Some(author) = comment.author() {
        div { "Comment from " #author }
      }
      div { "Comment Status: " #comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi") }
      #if let Some(location) = comment.location() {
        div { "Referencing " #(format!("{:?}", location)) }
      }
      #if let Some(description) = comment.description() {
        div { #(markdown::from_string(description)) }
      }
    }
  }
}
