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
  fn tab() -> Tab;
}

pub struct RepositoryWrapper<'a, R: RepositoryTab>(pub &'a RepositoryContext, pub R);

impl<'a, R: RenderOnce + RepositoryTab> RenderOnce for RepositoryWrapper<'a, R> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let tab = R::tab();
    let RepositoryWrapper(context, content) = self;
    let path = context.requested_path.to_string_lossy().into_owned();
    html!(w, {
      ^super::RepositoriesHeader
      div.block {
        ^super::RepositoryHeader(&path, &context.repository)
        ^RepositoryWrapperTabs(tab, path, context.repository.head().unwrap().shorthand().unwrap().to_owned())
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
