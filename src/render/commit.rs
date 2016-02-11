use std::fmt;
use git2::{ self, Oid };
use maud::{ RenderOnce, PreEscaped };
use maud_pulldown_cmark::Markdown;
use commit_tree;
use chrono::naive::datetime::NaiveDateTime;

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: Oid) -> String {
  oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

fn summary<'a>(commit: &'a git2::Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.lines().nth(0))
}

fn non_summary<'a>(commit: &'a git2::Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.splitn(2, '\n').map(|l| if l.starts_with('\r') { &l[1..] } else { l }).nth(1))
}

renderers! {
  CommitStub(root: &'a str, commit: &'a git2::Commit<'a>) {
    li.commit-stub {
      a href={ ^root "/commit/" ^commit.id() } {
        span.id
          ^short(commit.id())
        " "
        @match summary(commit) {
          Some(summary) => ^summary,
          None => "<No summary provided>",
        }
      }
    }
  }

  CommitHeader(commit: &'a git2::Commit<'a>) {
    div.block-header {
      div.h2 {
        span.id ^short(commit.id())
        ^PreEscaped("&nbsp;")
        @match summary(commit) {
          Some(summary) => ^summary,
          None => "<No summary provided>",
        }
      }
      div.h3 {
        ^super::Signature(&commit.committer())
        span {
          "committed at "
          span.timestamp { ^NaiveDateTime::from_timestamp(commit.time().seconds(), 0) }
        }
      }
      @if (commit.author().name(), commit.author().email()) != (commit.committer().name(), commit.committer().email()) {
        div.h3 {
          ^super::Signature(&commit.author())
          span {
            "authored at "
            span.timestamp { ^NaiveDateTime::from_timestamp(commit.author().when().seconds(), 0) }
          }
        }
      }
    }
  }

  CommitDetails(commit: &'a git2::Commit<'a>) {
    div.commit.block {
      ^CommitHeader(commit)
      @if let Some(non_summary) = non_summary(commit) {
        @if !non_summary.is_empty() {
          div.block-details.message {
            ^Markdown::from_string(non_summary)
          }
        }
      }
    }
  }

  Commit(repo: &'a git2::Repository, commit: &'a git2::Commit<'a>) {
    ^CommitDetails(commit)
    ^super::DiffCommit(repo, commit)
  }
}

pub struct Commits<'repo, 'a>(pub &'a str, pub &'a str, pub commit_tree::CommitTree<'repo>);
impl<'repo, 'a> RenderOnce for Commits<'repo, 'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let Commits(root, reff, commits) = self;
    html!(w, {
      div.block {
        div.block-header {
          h3 { "Commits for ref " span.ref { ^reff } }
        }
        div.block-details {
          ^CommitTree(root, commits)
        }
      }
    })
  }
}

pub struct CommitTree<'repo, 'a>(pub &'a str, pub commit_tree::CommitTree<'repo>);
impl<'repo, 'a> RenderOnce for CommitTree<'repo, 'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let CommitTree(root, commits) = self;
    html!(w, {
      ul.no-dot {
        @for (commit, sub) in commits {
          ^CommitStub(root, &commit)
          @if !sub.is_empty() {
            li {
              input.expander type="checkbox" { }
              label { i.fa.fa-fw.chevron {} }
              ^CommitTree(root, sub)
            }
          }
        }
      }
    })
  }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Commit<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Commits }
}

impl<'a, 'b> super::repository_wrapper::RepositoryTab for Commits<'a, 'b> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Commits }
}
