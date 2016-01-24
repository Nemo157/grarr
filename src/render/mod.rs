use std::fmt;
use maud::Render;

#[macro_use]
mod macros;

mod style;
mod event;
mod ci_status;
mod review;
mod request;
mod comment;
mod analysis;
mod avatar;
mod commit;

pub use self::style::Style;
pub use self::event::{ EventRenderer, EventsRenderer };
pub use self::request::{ RequestRenderer, RequestStubRenderer };
pub use self::review::{ ReviewRenderer, ReviewsRenderer };
pub use self::comment::{ CommentRenderer };
pub use self::ci_status::{ CIStatusRenderer };
pub use self::analysis::{ AnalysisRenderer };
pub use self::avatar::{ Avatar };
pub use self::commit::{ CommitsRenderer };

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
