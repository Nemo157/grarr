use std::str;
use std::cmp::{ Ord, PartialOrd, Ordering };
use std::fmt;
use super::fa::{ FA, FAM };
use git2::{ self, ObjectType };
use std::path::{ self, Path, Component };
use referenced_commit::ReferencedCommit;
use tree_entry::TreeEntryContext;

renderers! {
  TreeEntryStub(parent: &'a TreeEntryContext<'a>, entry: &'a git2::TreeEntry<'a>) {
    @if let Some(name) = entry.name() {
      li {
        @match entry.kind() {
          Some(ObjectType::Tree) => {
            ^FAM::Li(FA::Sitemap)
            a href={ "/" ^parent.repo_path "/tree/" ^parent.reff ^parent.entry_path.trim_right_matches('/') "/" ^name } { ^name }
          },
          Some(ObjectType::Blob) => {
            ^FAM::Li(FA::File)
            a href={ "/" ^parent.repo_path "/blob/" ^parent.reff ^parent.entry_path.trim_right_matches('/') "/" ^name } { ^name }
          },
          _ => {
            ^FAM::Li(FA::Question)
            ^name
          },
        }
      }
    }
  }

  TreeEntry(context: &'a TreeEntryContext<'a>) {
    div {
      @match context.entry.kind() {
        Some(ObjectType::Tree) => ^Tree(context.entry.as_tree().unwrap(), context),
        Some(ObjectType::Blob) => ^Blob(context.entry.as_blob().unwrap(), context),
        Some(ObjectType::Tag) => "Can't render ObjectType::Tag yet",
        Some(ObjectType::Commit) => "Can't render ObjectType::Commit yet",
        Some(ObjectType::Any) => "Can't render ObjectType::Any yet",
        None => "Can't render without an ObjectType",
      }
    }
  }

  Tree(tree: &'a git2::Tree<'a>, context: &'a TreeEntryContext<'a>) {
    div.block {
      div.block-header {
        h2 {
          ^FAM::FixedWidth(FA::File) " "
          span.path ^Components(context, false)
          " at "
          ^super::Reference(&context.commit)
        }
      }
      div.block-details {
        ul.fa-ul {
          @if let Some(parent) = context.parent() {
            li { ^FAM::Li(FA::LevelUp) a href={ "/" ^context.repo_path "/tree/" ^context.reff ^parent } { ".." } }
          }
          @for entry in tree.iter().collect::<Vec<_>>().tap(|v| v.sort_by_key(|e| Sorter(e.kind()))) {
            ^TreeEntryStub(context, &entry)
          }
        }
      }
    }
  }

  Blob(blob: &'a git2::Blob<'a>, context: &'a TreeEntryContext<'a>) {
    div.block {
      div.block-header {
        h2 {
          ^FAM::FixedWidth(FA::File) " "
          span.path ^Components(context, true)
          " at "
          ^super::Reference(&context.commit)
        }
      }
      pre.block-details {
        @match blob.is_binary() {
          true => code { "Binary file" },
          false => code class={ "hljs lang-" ^context.extension().unwrap_or("") } {
            @for (i, line) in str::from_utf8(blob.content()).unwrap().lines().enumerate() {
              div.line {
                a.line-num id={ "L" ^(i + 1) } href={ "#L" ^(i + 1) } data-line-num=^(format!("{: >4}", i + 1)) { }
                span.text ^line
              }
            }
          },
        }
      }
      ^super::HighlightJS
    }
  }
}

pub struct Components<'a>(pub &'a TreeEntryContext<'a>, pub bool);

impl<'a> ::maud::RenderOnce for Components<'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let Components(context, is_blob) = self;
    try!(html!(w, { a.path-component href={ "/" ^context.repo_path "/tree/" ^context.reff } ^context.repo_path }));
    let mut parent = "/".to_owned();
    let components: Vec<_> = context.entry_path.split_terminator('/').collect();
    for i in 0..components.len() {
      let component = components[i];
      if component == "" {
        continue;
      } else if is_blob && i == components.len() - 1 {
        try!(html!(w, { "/" a.path-component href={ "/" ^context.repo_path "/blob/" ^context.reff ^parent ^component } ^component }));
      } else {
        try!(html!(w, { "/" a.path-component href={ "/" ^context.repo_path "/tree/" ^context.reff ^parent ^component } ^component }));
        parent.push_str(component);
        parent.push('/');
      }
    }
    Ok(())
  }
}

trait Tapable {
  fn tap<F: Fn(&mut Self)>(self, f: F) -> Self;
}

impl<T: Sized> Tapable for T {
  fn tap<F: Fn(&mut T)>(mut self, f: F) -> T {
    f(&mut self);
    self
  }
}

#[derive(Eq, PartialEq)]
struct Sorter(Option<ObjectType>);

impl Ord for Sorter {
  #[cfg_attr(feature = "clippy", allow(match_same_arms))]
  fn cmp(&self, other: &Self) -> Ordering {
    match (self.0, other.0) {
      (x, y) if x == y => Ordering::Equal,
      (Some(ObjectType::Tree), _) => Ordering::Less,
      (_, Some(ObjectType::Tree)) => Ordering::Greater,
      (Some(ObjectType::Tag), _) => Ordering::Less,
      (_, Some(ObjectType::Tag)) => Ordering::Greater,
      (Some(ObjectType::Commit), _) => Ordering::Less,
      (_, Some(ObjectType::Commit)) => Ordering::Greater,
      (Some(ObjectType::Blob), _) => Ordering::Less,
      (_, Some(ObjectType::Blob)) => Ordering::Greater,
      (Some(ObjectType::Any), _) => Ordering::Less,
      (_, Some(ObjectType::Any)) => Ordering::Greater,
      (None, None) => Ordering::Equal,
    }
  }
}

impl PartialOrd for Sorter {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a TreeEntry<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Files }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Tree<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Files }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Blob<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Files }
}
