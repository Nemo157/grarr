use std::fmt;
use maud::{ Render, RenderMut, RenderOnce };
use super::Style;

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