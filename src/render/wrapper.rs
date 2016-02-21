use std::fmt;
use maud::{ Render, RenderOnce };
use super::Style;

pub struct Wrapper<T>(pub T);

impl<T: Render> Render for Wrapper<T> {
  fn render(&self, mut w: &mut fmt::Write) -> fmt::Result {
    html!(w, {
      html {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1" {}
          meta name="referrer" content="none-when-downgrade" {}
          ^Style
        }
        body {
          ^self.0
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
          meta name="viewport" content="width=device-width, initial-scale=1" {}
          ^Style
        }
        body {
          ^self.0
        }
      }
    })
  }
}
