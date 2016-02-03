use std::fmt;
use maud::RenderOnce;
use super::fa::{ FA };
use repository_context::RepositoryContext;

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

pub struct RepositoryWrapper<'a, R: RepositoryTab>(pub &'a RepositoryContext, pub R);

impl<'a, R: RenderOnce + RepositoryTab> RenderOnce for RepositoryWrapper<'a, R> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let tab = R::tab();
    let RepositoryWrapper(context, content) = self;
    let requested_path = context.requested_path.to_string_lossy();
    let canonical_path = context.canonical_path.to_string_lossy();
    html!(w, {
      ^FA::LevelUp " " a href="/" { "Repositories" }
      h1 {
        ^FA::GitSquare " "
        a href={ "/" ^requested_path } { ^requested_path }
        @if requested_path != canonical_path {
          " "
          small {
            "(alias of " a href={ "/" ^canonical_path } { ^canonical_path } ")"
          }
        }
      }
      .repository {
        .tabs {
          div class={ "overview" @if tab == Tab::Overview { " selected" } } { a href={ "/" ^requested_path } { "Overview" } }
          div class={ "files" @if tab == Tab::Files { " selected" } } { a href={ "/" ^requested_path "/tree" } { "Files" } }
          div class={ "commits" @if tab == Tab::Commits { " selected" } } { a href={ "/" ^requested_path "/commits" } { "Commits" } }
          div class={ "reviews" @if tab == Tab::Reviews { " selected" } } { a href={ "/" ^requested_path "/reviews" } { "Reviews" } }
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
