use std::fmt;
use maud::RenderOnce;
use { RepositoryContext };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tab {
  Overview,
  Files,
  Commits,
  Reviews,
}

pub trait RepositoryTab {
  fn tab() -> Option<Tab>;
}

pub struct RepositoryWrapper<'a, R: RepositoryTab>(pub &'a RepositoryContext, pub R);

impl<'a, R: RenderOnce + RepositoryTab> RenderOnce for RepositoryWrapper<'a, R> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let tab = R::tab();
    let RepositoryWrapper(context, content) = self;
    html!(w, {
      div.block {
        ^super::RepositoryHeader(&context.path, &context.repository)
        ^RepositoryWrapperTabs(&tab, &context.path, context.repository.head().unwrap().shorthand().unwrap())
      }
      ^content
    })
  }
}

renderers! {
  RepositoryWrapperTabs(tab: &'a Option<Tab>, path: &'a str, head: &'a str) {
    div.tabs {
      div class={ "overview" @if *tab == Some(Tab::Overview) { " selected" } } { a href={ "/" ^path } { "Overview" } }
      div class={ "files" @if *tab == Some(Tab::Files) { " selected" } } { a href={ "/" ^path "/tree/" ^head } { "Files" } }
      div class={ "commits" @if *tab == Some(Tab::Commits) { " selected" } } { a href={ "/" ^path "/commits/" ^head } { "Commits" } }
      div class={ "reviews" @if *tab == Some(Tab::Reviews) { " selected" } } { a href={ "/" ^path "/reviews" } { "Reviews" } }
    }
  }
}
