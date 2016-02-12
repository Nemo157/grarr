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
    div.block {
      ^CommitHeader(root, commit)
    }
  }

  CommitHeader(root: &'a str, commit: &'a git2::Commit<'a>) {
    div.block-header {
      div.row {
        @if commit.author().email() == commit.committer().email() {
          @if let Some(email) = commit.author().email() {
            ^super::Avatar(email, &commit.author().name())
          }
        } @else {
          div.column {
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
            a href={ ^root "/commit/" ^commit.id() } {
              span.id ^short(commit.id())
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

  CommitDetails(root: &'a str, commit: &'a git2::Commit<'a>) {
    div.commit.block {
      ^CommitHeader(root, commit)
      @if let Some(non_summary) = non_summary(commit) {
        @if !non_summary.is_empty() {
          div.block-details.message {
            ^Markdown::from_string(non_summary)
          }
        }
      }
    }
  }

  Commit(root: &'a str, repo: &'a git2::Repository, commit: &'a git2::Commit<'a>) {
    ^CommitDetails(root, commit)
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
      }
      ^CommitTree(root, commits, &mut 0)
    })
  }
}

pub struct CommitTree<'repo, 'a>(pub &'a str, pub commit_tree::CommitTree<'repo>, pub &'a mut u32);
impl<'repo, 'a> RenderOnce for CommitTree<'repo, 'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let CommitTree(root, commits, id) = self;
    *id = *id + 1;
    html!(w, {
      div.commits {
        @for (commit, sub) in commits {
          ^CommitStub(root, &commit)
          @if !sub.is_empty() {
            div.subtree {
              input.expander disabled?=(sub.len() == 1) id={ "commits-expander-" ^id } type="checkbox" checked? { }
              label for={ "commits-expander-" ^id } { i.fa.fa-fw.chevron {} }
              ^CommitTree(root, sub, id)
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
