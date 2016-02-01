use std::fmt;
use maud::RenderOnce;
use super::fa::{ FA };

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Tab {
  Overview,
  Files,
  Commits,
  Reviews,
}

pub struct RepositoryWrapper<'a, R>(pub &'a str, pub &'a str, pub Tab, pub R);

impl<'a, R: RenderOnce> RenderOnce for RepositoryWrapper<'a, R> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let RepositoryWrapper(name, actual, tab, content) = self;
    html!(w, {
      #FA::LevelUp " " a href="/" { "Repositories" }
      h1 {
        #FA::GitSquare " "
        a href={ "/" #name } { #name }
        #if name != actual {
          " "
          small {
            "(alias of " a href={ "/" #actual } { #actual } ")"
          }
        }
      }
      .repository {
        .tabs {
          div class={ "overview" #{ if tab == Tab::Overview { " selected" } else { "" } } } { a href={ "/" #name } { "Overview" } }
          div class={ "files" #{ if tab == Tab::Files { " selected" } else { "" } } } { a href={ "/" #name "/tree" } { "Files" } }
          div class={ "commits" #{ if tab == Tab::Commits { " selected" } else { "" } } } { a href={ "/" #name "/commits" } { "Commits" } }
          div class={ "reviews" #{ if tab == Tab::Reviews { " selected" } else { "" } } } { a href={ "/" #name "/reviews" } { "Reviews" } }
        }
        div class={ "content " #match tab {
          Tab::Overview => "overview",
          Tab::Files => "files",
          Tab::Commits => "commits",
          Tab::Reviews => "reviews",
        } } {
          #content
        }
      }
    })
  }
}
