use std::fmt;
use maud::{ Render, RenderOnce };
use super::Style;
use super::fa::FA;
use settings::Settings;

renderers! {
  Header {
    div.block {
      div.block-header {
        h1 {
          a href="/" { "Repositories" }
          small.float-right {
            a href="/-/settings" { ^FA::Cog }
            " "
            a href="/-/about" { ^FA::Info }
          }
        }
      }
    }
  }
}

pub struct Wrapper<T>(pub T, pub Settings);

impl<T: Render> Render for Wrapper<T> {
  fn render(&self, mut w: &mut fmt::Write) -> fmt::Result {
    html!(w, {
      html {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1" {}
          meta name="referrer" content="none-when-downgrade" {}
          ^Style(&self.1)
        }
        body {
          ^Header
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
          ^Style(&self.1)
        }
        body {
          ^Header
          ^self.0
        }
      }
    })
  }
}
