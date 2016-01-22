use std::fmt;
use maud::Render;

#[macro_use]
mod macros;

mod style;
mod review;
mod request;
mod comment;

pub use self::style::Style;
pub use self::review::ReviewRenderer;
pub use self::review::ReviewsRenderer;
pub use self::request::RequestRenderer;
pub use self::comment::CommentRenderer;

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
