use std::fmt;
use maud::Render;

#[macro_use]
mod macros;

mod style;
mod status;
mod review;
mod request;
mod comment;
mod analysis;

pub use self::style::Style;
pub use self::request::RequestRenderer;
pub use self::review::{ ReviewRenderer, ReviewsRenderer };
pub use self::comment::{ CommentRenderer, CommentsRenderer };
pub use self::status::{ CIStatusRenderer, CIStatusesRenderer };
pub use self::analysis::{ AnalysisRenderer, AnalysesRenderer };

pub struct Wrapper<T: Render>(pub T);

impl<T: Render> Render for Wrapper<T> {
  fn render(&self, mut w: &mut fmt::Write) -> fmt::Result {
    html!(w, {
      html {
        head {
          #Style
        }
        body {
          #(self.0)
        }
      }
    })
  }
}
