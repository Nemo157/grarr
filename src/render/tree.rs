use std::str;
use std::cmp::{ Ord, PartialOrd, Ordering };
use std::fmt;
use super::fa::{ FA, FAM };
use git2::{ self, ObjectType };
use std::path::{ self, Path, Component };
use referenced_commit::ReferencedCommit;

renderers! {
  TreeEntryStub(root: &'a str, entry: &'a git2::TreeEntry<'a>) {
    @if let Some(name) = entry.name() {
      li {
        @match entry.kind() {
          Some(ObjectType::Tree) => ^FAM::Li(FA::Sitemap),
          Some(ObjectType::Blob) => ^FAM::Li(FA::File),
          _ => ^FAM::Li(FA::Question),
        }
        a href={ ^root "/" ^name } { ^name }
      }
    }
  }

  TreeEntry(root: &'a str, path: &'a Path, entry: &'a git2::Object<'a>, commit: &'a ReferencedCommit<'a>) {
    div {
      @match entry.kind() {
        Some(ObjectType::Tree) => ^Tree(root, path, entry.as_tree().unwrap(), commit),
        Some(ObjectType::Blob) => ^Blob(root, path, entry.as_blob().unwrap(), commit),
        Some(ObjectType::Tag) => "Can't render ObjectType::Tag yet",
        Some(ObjectType::Commit) => "Can't render ObjectType::Commit yet",
        Some(ObjectType::Any) => "Can't render ObjectType::Any yet",
        None => "Can't render without an ObjectType",
      }
    }
  }

  Tree(root: &'a str, path: &'a Path, tree: &'a git2::Tree<'a>, commit: &'a ReferencedCommit<'a>) {
    div.block {
      div.block-header {
        h2 {
          ^FAM::FixedWidth(FA::File) " "
          span.path ^Components(root, path.components())
          " at "
          ^super::Reference(commit)
        }
      }
      div.block-details {
        ul.fa-ul {
          @if path != Path::new("") {
            li { ^FAM::Li(FA::LevelUp) a href=^((root.to_owned() + "/" + path.parent().and_then(|p| p.to_str()).unwrap_or("")).trim_right_matches('/')) ".." }
          }
          @for entry in tree.iter().collect::<Vec<_>>().tap(|v| v.sort_by_key(|e| Sorter(e.kind()))) {
            ^TreeEntryStub(&(root.to_owned() + "/" + path.to_str().unwrap()).trim_right_matches('/'), &entry)
          }
        }
      }
    }
  }

  Blob(root: &'a str, path: &'a Path, blob: &'a git2::Blob<'a>, commit: &'a ReferencedCommit<'a>) {
    div.block {
      div.block-header {
        h2 {
          ^FAM::FixedWidth(FA::File) " "
          span.path ^Components(root, path.components())
          " at "
          ^super::Reference(commit)
        }
      }
      pre.block-details {
        @match blob.is_binary() {
          true => code { "Binary file" },
          false => code class={ "hljs lang-" ^path.extension().map(|s| s.to_string_lossy().into_owned()).unwrap_or("".to_owned()) } {
            @for (i, line) in str::from_utf8(blob.content()).unwrap().lines().enumerate() {
              div.line data-line-num=^(format!("{: >4}", i + 1)) {
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

pub struct Components<'a>(&'a str, pub path::Components<'a>);

impl<'a> ::maud::RenderOnce for Components<'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let mut root = self.0.to_owned();
    try!(html!(w, { a.path-component href={ ^root } "<root>" }));
    for component in self.1 {
      if let Component::Normal(component) = component {
        try!(html!(w, { "/" a.path-component href={ ^root "/" ^component.to_str().unwrap() } ^component.to_str().unwrap() }));
        root = root + "/" + component.to_str().unwrap();
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
