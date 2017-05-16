use maud::{ Render, Markup };
use { RepositoryContext };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tab {
    Overview,
    Files,
    Commits,
    Reviews,
}

pub struct RepositoryWrapper<'a, R: Render>(pub &'a RepositoryContext, pub R, pub Option<Tab>);

impl<'a, R: Render> Render for RepositoryWrapper<'a, R> {
    fn render(&self) -> Markup {
        let &RepositoryWrapper(ref context, ref content, ref tab) = self;
        html!({
            div.block {
                (super::RepositoryHeader(&context.path, &context.repository))
                (RepositoryWrapperTabs(&tab, &context.path, context.repository.head().unwrap().shorthand().unwrap()))
            }
            (content)
        })
    }
}

pub fn RepositoryWrapperTabs(tab: &Option<Tab>, path: &str, head: &str) -> ::maud::Markup {
    html! {
        div.tabs {
            div class={ "overview" @if *tab == Some(Tab::Overview) { " selected" } } { a href={ "/" (path) } { "Overview" } }
            div class={ "files" @if *tab == Some(Tab::Files) { " selected" } } { a href={ "/" (path) "/tree/" (head) } { "Files" } }
            div class={ "commits" @if *tab == Some(Tab::Commits) { " selected" } } { a href={ "/" (path) "/commits/" (head) } { "Commits" } }
            div class={ "reviews" @if *tab == Some(Tab::Reviews) { " selected" } } { a href={ "/" (path) "/reviews" } { "Reviews" } }
        }
    }
}
