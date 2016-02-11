use std::fmt;
use maud::RenderOnce;
use super::fa::{ FA };
use { RepositoryContext, RepositoryExtension };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tab {
  Overview,
  Files,
  Commits,
  Reviews,
}

impl Tab {
  fn css_class(&self) -> &'static str {
    match *self {
      Tab::Overview => "overview",
      Tab::Files => "files",
      Tab::Commits => "commits",
      Tab::Reviews => "reviews",
    }
  }
}

pub trait RepositoryTab {
  fn tab() -> Tab;
}

pub struct RepositoryWrapper<'a, R: RepositoryTab>(pub &'a RepositoryContext, pub R);

impl<'a, R: RenderOnce + RepositoryTab> RenderOnce for RepositoryWrapper<'a, R> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let tab = R::tab();
    let RepositoryWrapper(context, content) = self;
    let requested_path = context.requested_path.to_string_lossy().into_owned();
    let canonical_path = context.canonical_path.to_string_lossy().into_owned();
    html!(w, {
      ^FA::LevelUp " " a href="/" { "Repositories" }
      div.repository-header {
        h1 {
          @match context.repository.origin_url() {
            Some(_) => ^FA::CodeFork,
            None => ^FA::Home,
          }
          " "
          a href={ "/" ^requested_path } { ^requested_path }
        }
        @if requested_path != canonical_path {
          h4 {
            "(alias of " a href={ "/" ^canonical_path } { ^canonical_path } ")"
          }
        }
        @if let Some(origin) = context.repository.origin_url() {
          h4 {
            "(fork of " ^super::MaybeLink(&origin, &origin) ")"
          }
        }
        ^RepositoryWrapperTabs(tab, requested_path, context.repository.head().unwrap().shorthand().unwrap().to_owned())
      }
      ^content
    })
  }
}

renderers! {
  RepositoryWrapperTabs(tab: Tab, requested_path: String, head: String) {
    div.tabs {
      div class={ "overview" @if tab == Tab::Overview { " selected" } } { a href={ "/" ^requested_path } { "Overview" } }
      div class={ "files" @if tab == Tab::Files { " selected" } } { a href={ "/" ^requested_path "/tree/" ^head } { "Files" } }
      div class={ "commits" @if tab == Tab::Commits { " selected" } } { a href={ "/" ^requested_path "/commits/" ^head } { "Commits" } }
      div class={ "reviews" @if tab == Tab::Reviews { " selected" } } { a href={ "/" ^requested_path "/reviews" } { "Reviews" } }
    }
  }
}
