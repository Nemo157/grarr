use std::str;
use std::fmt;
use git2::{ self, Commit, DiffFormat, Repository };
use maud::{ RenderOnce };

renderers! {
  DiffCommits(repo: &'a Repository, old_commit: &'a Option<&'a Commit<'a>>, new_commit: &'a Commit<'a>) {
    @match repo.diff_tree_to_tree(old_commit.map(|commit| commit.tree().unwrap()).as_ref(), Some(&new_commit.tree().unwrap()), None) {
      Ok(diff) => ^Diff(diff),
      Err(ref error) => ^super::Error(error),
    }
  }

  DiffCommit(repo: &'a Repository, commit: &'a Commit<'a>) {
    ^DiffCommits(repo, &commit.parents().nth(0).as_ref(), commit)
  }
}


pub struct Diff(pub git2::Diff);

impl RenderOnce for Diff {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let Diff(diff) = self;
    w.write_str("<div class=\"diff block\"><div class=\"block-details\"><pre><code>").unwrap();
    diff.print(DiffFormat::Patch, |_, _, line| {
      w.write_char(line.origin()).unwrap();
      w.write_char(' ').unwrap();
      w.write_str(str::from_utf8(line.content()).unwrap()).unwrap();
      true
    }).unwrap();
    w.write_str("</code></pre></div></div>").unwrap();
    html!(w, {
      link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.1.0/styles/solarized-light.min.css" {}
      script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.1.0/highlight.min.js" {}
      script { "hljs.initHighlightingOnLoad()" }
    }).unwrap();
    Ok(())
  }
}

