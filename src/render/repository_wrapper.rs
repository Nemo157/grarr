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

pub trait RepositoryTab {
  fn tab() -> Tab;
}

pub struct RepositoryWrapper<'a, R: RepositoryTab>(pub &'a str, pub &'a str, pub R);

impl<'a, R: RenderOnce + RepositoryTab> RenderOnce for RepositoryWrapper<'a, R> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let tab = R::tab();
    let RepositoryWrapper(name, actual, content) = self;
    html!(w, {
      ^FA::LevelUp " " a href="/" { "Repositories" }
      h1 {
        ^FA::GitSquare " "
        a href={ "/" ^name } { ^name }
        @if name != actual {
          " "
          small {
            "(alias of " a href={ "/" ^actual } { ^actual } ")"
          }
        }
      }
      .repository {
        .tabs {
          div class={ "overview" @if tab == Tab::Overview { " selected" } } { a href={ "/" ^name } { "Overview" } }
          div class={ "files" @if tab == Tab::Files { " selected" } } { a href={ "/" ^name "/tree" } { "Files" } }
          div class={ "commits" @if tab == Tab::Commits { " selected" } } { a href={ "/" ^name "/commits" } { "Commits" } }
          div class={ "reviews" @if tab == Tab::Reviews { " selected" } } { a href={ "/" ^name "/reviews" } { "Reviews" } }
        }
        div class={ "content " @match tab {
          Tab::Overview => "overview",
          Tab::Files => "files",
          Tab::Commits => "commits",
          Tab::Reviews => "reviews",
        } } {
          ^content
        }
      }
    })
  }
}
