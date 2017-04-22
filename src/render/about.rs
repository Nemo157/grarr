use { REVISION, DATE };
use super::utils::Markdown;

pub fn About() -> ::maud::Markup {
  html! {
    div.block {
      div.block-header h3 "About"
      div.block-details {
        (Markdown(include_str!("../../README.md")))
      }
    }
    div.block {
      div.block-header h3 "Version"
      div.block-details {
        "Website generated using "
        a href="https://git.nemo157.com/grarr" "grarr"
        " version "
        (env!("CARGO_PKG_VERSION"))
        @match (REVISION, DATE) {
          (Some(rev), None) => " (" a href={ "https://git.nemo157.com/grarr/commits/" (rev) } (rev) ")",
          (None, Some(date)) => " (" (date) ")",
          (Some(rev), Some(date)) => " (" a href={ "https://git.nemo157.com/grarr/commits/" (rev) } (rev) " " (date) ")",
          (None, None) => {},
        }
      }
    }
  }
}
