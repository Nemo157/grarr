use std::fmt;
use maud::{ Render, Markup };
use super::Style;
use super::fa::FA;
use settings::Settings;

pub fn Header() -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h1 {
          a href="/" { "Repositories" }
          small.float-right {
            a href="/-/settings" { (FA::Cog) }
            " "
            a href="/-/about" { (FA::Info) }
          }
        }
      }
    }
  }
}

pub struct Wrapper<T>(pub T, pub Settings);

impl<T: Render> Render for Wrapper<T> {
  fn render(&self) -> Markup {
    html!({
      html {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1" {}
          meta name="referrer" content="none-when-downgrade" {}
          (Style(&self.1))
        }
        body {
          (Header())
          (self.0)
        }
      }
    })
  }
}
