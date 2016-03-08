use { REVISION, DATE };
use maud_pulldown_cmark::Markdown;

renderers! {
  About {
    div.block {
      div.block-header h3 "About"
      div.block-details {
        ^Markdown::from_string(include_str!("../../README.md"))
      }
    }
    div.block {
      div.block-header h3 "Version"
      div.block-details {
        "Website generated using "
        a href="https://git.nemo157.com/grarr" "grarr"
        " version "
        ^(env!("CARGO_PKG_VERSION"))
        @match (REVISION, DATE) {
          (Some(rev), None) => " (" a href={ "https://git.nemo157.com/grarr/commits/" ^rev } ^rev ")",
          (None, Some(date)) => " (" ^date ")",
          (Some(rev), Some(date)) => " (" a href={ "https://git.nemo157.com/grarr/commits/" ^rev } ^rev " " ^date ")",
          (None, None) => {},
        }
      }
    }
  }
}
