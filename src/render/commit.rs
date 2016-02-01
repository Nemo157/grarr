use std::fmt;
use git2::{ Oid, Commit };
use maud::RenderOnce;
use maud_pulldown_cmark::markdown;
use commit_tree::CommitTree;

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: Oid) -> String {
  oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

fn summary<'a>(commit: &'a Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.lines().nth(0))
}

fn non_summary<'a>(commit: &'a Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.splitn(2, '\n').map(|l| if l.starts_with('\r') { &l[1..] } else { l }).nth(1))
}

renderers! {
  CommitStubRenderer(commit: &'a Commit<'a>) {
    li class="commit-stub" {
      a href={ "commits/" #commit.id() } {
        span class="id"
          #short(commit.id())
        " "
        #if let Some(summary) = summary(commit) {
          #summary
        }
        #if let None = summary(commit) {
          "<No summary provided>"
        }
      }
    }
  }

  CommitRenderer(commit: &'a Commit<'a>) {
    div class="commit" {
      h2 {
        span class="id" #short(commit.id())
        " "
        #if let Some(summary) = summary(commit) {
          #summary
        }
        #if let None = summary(commit) {
          "<No summary provided>"
        }
      }
      #if let Some(non_summary) = non_summary(commit) {
        div class="message" {
          #(markdown::from_string(non_summary))
        }
      }
    }
  }
}

pub struct CommitsRenderer<'repo>(pub CommitTree<'repo>);
impl<'repo> RenderOnce for CommitsRenderer<'repo> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let CommitsRenderer(commits) = self;
    html!(w, {
      ul {
        #for (commit, sub) in commits {
          #CommitStubRenderer(&commit)
          #if !sub.is_empty() {
            #CommitsRenderer(sub)
          }
        }
      }
    })
  }
}
