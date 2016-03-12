use std::fmt;
use git2;
use maud::{ RenderOnce, PreEscaped };
use maud_pulldown_cmark::Markdown;
use commit_tree;
use chrono::naive::datetime::NaiveDateTime;
use referenced_commit::ReferencedCommit;
use repository_context::RepositoryContext;
use super::reference;

fn summary<'a>(commit: &'a git2::Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.lines().nth(0))
}

fn non_summary<'a>(commit: &'a git2::Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.splitn(2, '\n').map(|l| if l.starts_with('\r') { &l[1..] } else { l }).nth(1))
}

renderers! {
  CommitStub(context: &'a RepositoryContext, commit: &'a git2::Commit<'a>) {
    div.block {
      ^CommitHeader(context, commit)
    }
  }

  CommitHeader(context: &'a RepositoryContext, commit: &'a git2::Commit<'a>) {
    div.block-header {
      div.row {
        @if commit.author().email() == commit.committer().email() {
          @if let Some(email) = commit.author().email() {
            ^super::Avatar(email, &commit.author().name())
          }
        } @else {
          div.column.fixed {
            @if let Some(email) = commit.author().email() {
              ^super::Avatar(email, &commit.author().name())
            }
            @if let Some(email) = commit.committer().email() {
              ^super::Avatar(email, &commit.committer().name())
            }
          }
        }
        div.column {
          div {
            a href={ "/" ^context.path "/commit/" ^commit.id() } {
              ^(super::reference::Commit(commit))
              " "
              @match summary(commit) {
                Some(summary) => ^summary,
                None => "<No summary provided>",
              }
            }
          }
          @if (commit.author().name(), commit.author().email()) == (commit.committer().name(), commit.committer().email()) {
            small {
              ^super::Signature(&commit.author(), &false)
              "committed at" ^PreEscaped("&nbsp;")
              span.timestamp { ^NaiveDateTime::from_timestamp(commit.time().seconds(), 0) }
            }
          } @else {
            small {
              ^super::Signature(&commit.author(), &false)
              "authored at" ^PreEscaped("&nbsp;")
              span.timestamp { ^NaiveDateTime::from_timestamp(commit.time().seconds(), 0) }
            }
            small {
              ^super::Signature(&commit.committer(), &false)
              "committed at" ^PreEscaped("&nbsp;")
              span.timestamp { ^NaiveDateTime::from_timestamp(commit.author().when().seconds(), 0) }
            }
          }
        }
      }
    }
  }

  CommitDetails(context: &'a RepositoryContext, commit: &'a git2::Commit<'a>) {
    div.commit.block {
      ^CommitHeader(context, commit)
      @if let Some(non_summary) = non_summary(commit) {
        @if !non_summary.is_empty() {
          div.block-details.message {
            ^Markdown::from_string(non_summary)
          }
        }
      }
    }
  }

  Commit(context: &'a RepositoryContext, commit: &'a git2::Commit<'a>) {
    ^CommitDetails(context, commit)
    ^super::DiffCommit(context, commit)
  }

  NextPage(context: &'a RepositoryContext, commit: &'a ReferencedCommit<'a>, next: &'a Option<&'a git2::Commit<'a>>) {
    div.block div.block-header.row {
      div.column.fixed {
      a href={
        "/"
        ^context.path
        "/commits/"
        ^commit.shorthand_or_id()
      } {
        "Back to beginning (" ^reference::Commit(&commit.commit) ")"
      }
      }
      div.column {}
      @if let Some(ref next) = *next {
        div.column.fixed {
          a.float-right href={
            "/"
            ^context.path
            "/commits/"
            ^commit.shorthand_or_id()
            "?start=" ^next.id()
          } {
            "Next page (" ^reference::Commit(next) ")"
          }
        }
      }
    }
  }
}

pub struct Commits<'repo, 'a>(pub &'a RepositoryContext, pub &'a ReferencedCommit<'a>, pub commit_tree::CommitTree<'repo>);
impl<'repo, 'a> RenderOnce for Commits<'repo, 'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let Commits(context, commit, mut commits) = self;
    let first = commits.next();
    let mut id = 0;
    html!(w, {
      @if let Some((first, mut sub)) = first {
        div.block {
          div.block-header {
            h3 {
              "Commits for ref " ^super::Reference(commit)
              @if first.id() != commit.commit.id() {
                small { " (showing from " ^reference::Commit(&first) ")" }
              }
            }
          }
        }
        ^CommitStub(context, &first)
        @if !sub.is_empty() {
          div.subtree {
            input.expander disabled?=(sub.len() == 1) id={ "commits-expander-" ^id } type="checkbox" checked? { }
            label for={ "commits-expander-" ^id } { i.fa.fa-fw.chevron {} }
            ^CommitTree(context, &mut sub, &mut id)
          }
        }
        ^CommitTree(context, &mut commits, &mut id)
        ^NextPage(context, commit, &commits.next_after())
      }
    })
  }
}

pub struct CommitTree<'a, 'repo: 'a>(pub &'a RepositoryContext, pub &'a mut commit_tree::CommitTree<'repo>, pub &'a mut u32);
impl<'repo, 'a> RenderOnce for CommitTree<'repo, 'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let CommitTree(context, commits, id) = self;
    *id = *id + 1;
    html!(w, {
      div.commits {
        @for (commit, mut sub) in commits {
          ^CommitStub(context, &commit)
          @if !sub.is_empty() {
            div.subtree {
              input.expander disabled?=(sub.len() == 1) id={ "commits-expander-" ^id } type="checkbox" checked? { }
              label for={ "commits-expander-" ^id } { i.fa.fa-fw.chevron {} }
              ^CommitTree(context, &mut sub, id)
            }
          }
        }
      }
    })
  }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Commit<'a> {
  fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Commits) }
}

impl<'a, 'b> super::repository_wrapper::RepositoryTab for Commits<'a, 'b> {
  fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Commits) }
}
