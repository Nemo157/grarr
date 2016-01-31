use std::fmt;
use maud::{ Render, RenderMut, RenderOnce };

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
mod repository;
mod not_found;
mod bad_request;
mod error;
mod fa;
mod tree;

pub use self::style::Style;
pub use self::event::{ EventRenderer, EventsRenderer };
pub use self::request::{ RequestRenderer, RequestStubRenderer };
pub use self::review::{ ReviewRenderer, ReviewsRenderer };
pub use self::comment::{ CommentRenderer };
pub use self::ci_status::{ CIStatusRenderer };
pub use self::analysis::{ AnalysisRenderer };
pub use self::avatar::{ Avatar };
pub use self::commit::{ CommitRenderer, CommitsRenderer };
pub use self::repository::{ RepositoryWrapper, RepositoryRenderer, RepositoriesRenderer, Tab };
pub use self::not_found::{ NotFoundRenderer };
pub use self::bad_request::{ BadRequestRenderer };
pub use self::error::{ ErrorRenderer };
pub use self::tree::{ RootTreeRenderer, TreeEntryRenderer };

pub struct Wrapper<T>(pub T);

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

impl<T: RenderMut> RenderMut for Wrapper<T> {
  fn render_mut(&mut self, mut w: &mut fmt::Write) -> fmt::Result {
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

impl<T: RenderOnce> RenderOnce for Wrapper<T> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
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
