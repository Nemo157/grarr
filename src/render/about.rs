use maud_pulldown_cmark::Markdown;

renderers! {
  About {
    div.block {
      div.block-header h3 "About"
      div.block-details {
        ^Markdown::from_string(include_str!("../../README.md"))
      }
    }
  }
}
