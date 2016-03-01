use std::mem;
use std::path::PathBuf;
use std::cell::RefCell;
use git2::{ self, Commit, Repository };
use maud::{ RenderOnce };

renderers! {
  DiffCommits(repo: &'a Repository, old_commit: &'a Option<&'a Commit<'a>>, new_commit: &'a Commit<'a>) {
    @match repo.diff_tree_to_tree(old_commit.map(|commit| commit.tree().unwrap()).as_ref(), Some(&new_commit.tree().unwrap()), None) {
      Ok(ref diff) => ^Diff(diff),
      Err(ref error) => ^super::Error(error),
    }
  }

  DiffCommit(repo: &'a Repository, commit: &'a Commit<'a>) {
    ^DiffCommits(repo, &commit.parents().nth(0).as_ref(), commit)
  }

  DiffHeader(delta: &'a DiffDelta) {
    div.block-header {
      @match (delta.status.0, delta.new_file.as_ref(), delta.old_file.as_ref()) {
        (git2::Delta::Added, Some(ref new_file), _) => {
          h3 { span { "Added " span.filename ^new_file.to_string_lossy() } }
        },
        (git2::Delta::Deleted, _, Some(ref old_file)) => {
          h3 { span { "Deleted " span.filename ^old_file.to_string_lossy() } }
        },
        (git2::Delta::Modified, Some(ref new_file), Some(ref old_file)) if old_file == new_file => {
          h3 { span { "Modified " span.filename ^new_file.to_string_lossy() } }
        },
        (git2::Delta::Modified, Some(ref new_file), Some(ref old_file)) if old_file != new_file => {
          h3 { span { "Modified " span.filename ^new_file.to_string_lossy() "(Previously " span.filename ^old_file.to_string_lossy() ")" } }
        },
        (git2::Delta::Renamed, Some(ref new_file), Some(ref old_file)) => {
          h3 { span { "Renamed " span.filename ^old_file.to_string_lossy() " to " span.filename ^new_file.to_string_lossy() } }
        },
        (git2::Delta::Copied, Some(ref new_file), Some(ref old_file)) => {
          h3 { span { "Copied " span.filename ^old_file.to_string_lossy() " to " span.filename ^new_file.to_string_lossy() } }
        },
        (status, ref new_file, ref old_file) =>  ^(format!("{:?} ({:?} -> {:?}) (should not happen)", status, old_file, new_file))
      }
    }
  }

  DiffDetails(extension: Option<String>, hunks: Vec<(DiffHunk, Vec<DiffLine>)>) {
    pre.block-details code class={ "hljs lang-" ^extension.unwrap_or("".to_owned()) } {
      @if hunks.is_empty() {
        div.line.hunk-header span.text "No content"
      }
      @for (hunk, lines) in hunks {
        div.line.hunk-header
          span.text ^hunk.header.unwrap()
        @for line in lines {
          @match (line.origin, line.content) {
            (Origin::LineContext, Some(ref content)) => {
              div.line.context
                data-old-line-num={ @if let Some(num) = line.old_lineno { ^(format!("{: >4}", num)) } @else { "    " } }
                data-new-line-num={ @if let Some(num) = line.new_lineno { ^(format!("{: >4}", num)) } @else { "    " } }
                span.text ^content
            },
            (Origin::LineAddition, Some(ref content)) => {
              div.line.addition
                data-old-line-num={ @if let Some(num) = line.old_lineno { ^(format!("{: >4}", num)) } @else { "    " } }
                data-new-line-num={ @if let Some(num) = line.new_lineno { ^(format!("{: >4}", num)) } @else { "    " } }
                span.text ^content
            },
            (Origin::LineDeletion, Some(ref content)) => {
              div.line.deletion
                data-old-line-num={ @if let Some(num) = line.old_lineno { ^(format!("{: >4}", num)) } @else { "    " } }
                data-new-line-num={ @if let Some(num) = line.new_lineno { ^(format!("{: >4}", num)) } @else { "    " } }
                span.text ^content
            },
            (Origin::AddEOF, _) => {
              div.line.add-eof
                span.text "Added EOF"
            },
            (Origin::RemoveEOF, _) => {
              div.line.remove-eof
                span.text "Removed EOF"
            },
            (Origin::LineBinary, _) => {
              div.line.binary
                span.text "Binary file changed"
            },
            (Origin::ContextEOF, _) | (Origin::FileHeader, _) | (Origin::HunkHeader, _) => {
            },
            (_, _) => {
              "UNREACHABLE"
            }
          }
        }
      }
    }
  }

  Diff(diff: &'a git2::Diff<'a>) {
    @for (delta, hunks) in group(diff).unwrap() {
      div.diff.block {
        ^DiffHeader(&delta)
        ^DiffDetails(delta.new_file.or(delta.old_file).and_then(|path| path.extension().map(|s| s.to_string_lossy().into_owned())), hunks)
      }
    }
    ^super::HighlightJS
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Delta(pub git2::Delta);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Origin {
  LineContext,
  LineAddition,
  LineDeletion,
  ContextEOF,
  AddEOF,
  RemoveEOF,
  FileHeader,
  HunkHeader,
  LineBinary,
}

impl From<char> for Origin {
  fn from(c: char) -> Origin {
    match c {
      ' ' => Origin::LineContext,
      '+' => Origin::LineAddition,
      '-' => Origin::LineDeletion,
      '=' => Origin::ContextEOF,
      '>' => Origin::AddEOF,
      '<' => Origin::RemoveEOF,
      'F' => Origin::FileHeader,
      'H' => Origin::HunkHeader,
      'B' => Origin::LineBinary,
      _ => panic!(),
    }
  }
}

#[derive(Eq, PartialEq, Debug)]
pub struct DiffDelta {
  pub status: Delta,
  pub old_file: Option<PathBuf>,
  pub new_file: Option<PathBuf>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DiffHunk {
  pub old_start: u32,
  pub old_lines: u32,
  pub new_start: u32,
  pub new_lines: u32,
  pub header: Option<String>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DiffLine {
  pub old_lineno: Option<u32>,
  pub new_lineno: Option<u32>,
  pub num_lines: u32,
  pub content_offset: i64,
  pub content: Option<String>,
  pub origin: Origin,
}

impl<'a> From<git2::DiffDelta<'a>> for DiffDelta {
  fn from(delta: git2::DiffDelta<'a>) -> DiffDelta {
    DiffDelta {
      status: Delta(delta.status()),
      old_file: delta.old_file().path().map(|p| p.to_owned()),
      new_file: delta.new_file().path().map(|p| p.to_owned()),
    }
  }
}

impl<'a> From<git2::DiffHunk<'a>> for DiffHunk {
  fn from(hunk: git2::DiffHunk<'a>) -> DiffHunk {
    DiffHunk {
      old_start: hunk.old_start(),
      old_lines: hunk.old_lines(),
      new_start: hunk.new_start(),
      new_lines: hunk.new_lines(),
      header: String::from_utf8(hunk.header().into()).ok(),
    }
  }
}

impl<'a> From<git2::DiffLine<'a>> for DiffLine {
  fn from(line: git2::DiffLine<'a>) -> DiffLine {
    DiffLine {
      old_lineno: line.old_lineno(),
      new_lineno: line.new_lineno(),
      num_lines: line.num_lines(),
      content_offset: line.content_offset(),
      content: String::from_utf8(line.content().into()).ok(),
      origin: line.origin().into(),
    }
  }
}

#[cfg_attr(feature = "clippy", allow(type_complexity))] // This is temporary till I figure out a nicer way to do this without all the allocation
fn group(diff: &git2::Diff) -> Result<Vec<(DiffDelta, Vec<(DiffHunk, Vec<DiffLine>)>)>, git2::Error> {
  let mut deltas = Vec::new();
  let hunks = RefCell::new(Vec::new());
  let lines = RefCell::new(Vec::new());
  let last_delta = RefCell::new(None);
  let last_hunk = RefCell::new(None);
  try!(diff.foreach(
    &mut |delta, _progress| {
      if let Some(last_hunk) = last_hunk.borrow_mut().take() {
        let mut new_lines = vec![];
        mem::swap(&mut *lines.borrow_mut(), &mut new_lines);
        hunks.borrow_mut().push((last_hunk, new_lines));
      }
      if let Some(last_delta) = last_delta.borrow_mut().take() {
        let mut new_hunks = vec![];
        mem::swap(&mut *hunks.borrow_mut(), &mut new_hunks);
        deltas.push((last_delta, new_hunks));
      }
      *last_delta.borrow_mut() = Some(delta.into());
      true
    },
    None,
    Some(&mut |_delta, hunk| {
      if let Some(last_hunk) = last_hunk.borrow_mut().take() {
        let mut new_lines = vec![];
        mem::swap(&mut *lines.borrow_mut(), &mut new_lines);
        hunks.borrow_mut().push((last_hunk, new_lines));
      }
      *last_hunk.borrow_mut() = Some(hunk.into());
      true
    }),
    Some(&mut |_delta, _hunk, line| {
      lines.borrow_mut().push(line.into());
      true
    })));
  if let Some(last_hunk) = last_hunk.into_inner() {
    hunks.borrow_mut().push((last_hunk, lines.into_inner()));
  }
  if let Some(last_delta) = last_delta.into_inner() {
    deltas.push((last_delta, hunks.into_inner()));
  }
  Ok(deltas)
}

impl Eq for Delta { }
impl PartialEq<Delta> for Delta {
  fn eq(&self, other: &Delta) -> bool {
    match (self.0, other.0) {
      (git2::Delta::Unmodified, git2::Delta::Unmodified) => true,
      (git2::Delta::Added, git2::Delta::Added) => true,
      (git2::Delta::Deleted, git2::Delta::Deleted) => true,
      (git2::Delta::Modified, git2::Delta::Modified) => true,
      (git2::Delta::Renamed, git2::Delta::Renamed) => true,
      (git2::Delta::Copied, git2::Delta::Copied) => true,
      (git2::Delta::Ignored, git2::Delta::Ignored) => true,
      (git2::Delta::Untracked, git2::Delta::Untracked) => true,
      (git2::Delta::Typechange, git2::Delta::Typechange) => true,
      (git2::Delta::Unreadable, git2::Delta::Unreadable) => true,
      (git2::Delta::Conflicted, git2::Delta::Conflicted) => true,
      (_, _) => false,
    }
  }
}
