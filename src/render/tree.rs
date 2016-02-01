use std::str;
use std::cmp::{ Ord, PartialOrd, Ordering };
use std::fmt;
use super::fa::{ FA, FAM };
use git2::{ Tree, TreeEntry, ObjectType, Repository, Blob };
use std::path::{ Path, PathBuf, Components, Component };

renderers! {
  TreeEntryStubRenderer(root: &'a str, entry: &'a TreeEntry<'a>) {
    #if let Some(name) = entry.name() {
      li {
        #{match entry.kind() {
          Some(ObjectType::Tree) => FAM::Li(FA::Sitemap),
          Some(ObjectType::Blob) => FAM::Li(FA::File),
          _ => FAM::Li(FA::Question),
        } }
        a href={ #root "/" #name } { #name }
      }
    }
  }

  TreeEntryRenderer(repo: &'a Repository, root: &'a str, path: &'a Path, entry: &'a TreeEntry<'a>) {
    div {
      #if let Some(ObjectType::Tree) = entry.kind() {
        #TreeRenderer(root, path, entry.to_object(repo).unwrap().as_tree().unwrap())
      }
      #if let Some(ObjectType::Blob) = entry.kind() {
        #BlobRenderer(root, path, entry.to_object(repo).unwrap().as_blob().unwrap())
      }
      #if let Some(ObjectType::Tag) = entry.kind() {
        "Can't render ObjectType::Tag yet"
      }
      #if let Some(ObjectType::Commit) = entry.kind() {
        "Can't render ObjectType::Commit yet"
      }
      #if let Some(ObjectType::Any) = entry.kind() {
        "Can't render ObjectType::Any yet"
      }
      #if let None = entry.kind() {
        "Can't render without an ObjectType"
      }
    }
  }

  RootTreeRenderer(root: &'a str, tree: &'a Tree<'a>) {
    h2 class="path" { #ComponentsRenderer(root, PathBuf::from("/").components()) }
    ul class="fa-ul" {
      #for entry in tree.iter().collect::<Vec<_>>().tap(|v| v.sort_by_key(|e| Sorter(e.kind()))) {
        #TreeEntryStubRenderer(root, &entry)
      }
    }
  }

  TreeRenderer(root: &'a str, path: &'a Path, tree: &'a Tree<'a>) {
    h2 class="path" { #ComponentsRenderer(root, path.components()) }
    ul class="fa-ul" {
      li { #(FAM::Li(FA::LevelUp)) a href=#((root.to_string() + path.parent().and_then(|p| p.to_str()).unwrap_or("")).trim_right_matches('/')) ".." }
      #for entry in tree.iter().collect::<Vec<_>>().tap(|v| v.sort_by_key(|e| Sorter(e.kind()))) {
        #TreeEntryStubRenderer(&(root.to_string() + path.to_str().unwrap()), &entry)
      }
    }
  }

  BlobRenderer(root: &'a str, path: &'a Path, blob: &'a Blob<'a>) {
    h2 class="path" { #ComponentsRenderer(root, path.components()) }
    ul class="fa-ul" {
      li { #(FAM::Li(FA::LevelUp)) a href=#((root.to_string() + path.parent().and_then(|p| p.to_str()).unwrap_or("")).trim_right_matches('/')) ".." }
    }
    #if blob.is_binary() {
      pre { code { "Binary file" } }
    }
    #if !blob.is_binary() {
      pre { code { #(str::from_utf8(blob.content()).unwrap()) } }
      link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.1.0/styles/solarized-light.min.css" {}
      script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.1.0/highlight.min.js" {}
      script { "hljs.initHighlightingOnLoad()" }
    }
  }
}

pub struct ComponentsRenderer<'a>(&'a str, pub Components<'a>);

impl<'a> ::maud::RenderOnce for ComponentsRenderer<'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let mut root = self.0.to_string();
    for component in self.1 {
      match component {
        Component::RootDir => {
          try!(html!(w, { a class="path-component" href={ #root } "<root>" }));
        },
        Component::Normal(component) => {
          try!(html!(w, { "/" a class="path-component" href={ #root "/" #component.to_str().unwrap() } #component.to_str().unwrap() }));
          root = root + "/" + component.to_str().unwrap();
        },
        _ => {
        },
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