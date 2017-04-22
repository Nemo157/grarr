use std::str;
use std::cmp::{ Ord, PartialOrd, Ordering };
use std::fmt;
use super::fa::{ FA, FAM };
use git2::{ self, ObjectType };
use tree_entry::TreeEntryContext;
use maud::{ Render };

pub fn TreeEntryStub(parent: &TreeEntryContext, entry: &git2::TreeEntry) -> ::maud::Markup {
  html! {
    @if let Some(name) = entry.name() {
      li {
        @match entry.kind() {
          Some(ObjectType::Tree) => {
            (FAM::Li(FA::Sitemap))
            a href={ "/" (parent.repo_path) "/tree/" (parent.reff) (parent.entry_path.trim_right_matches('/')) "/" (name) } { (name) }
          },
          Some(ObjectType::Blob) => {
            (FAM::Li(FA::File))
            a href={ "/" (parent.repo_path) "/blob/" (parent.reff) (parent.entry_path.trim_right_matches('/')) "/" (name) } { (name) }
          },
          _ => {
            (FAM::Li(FA::Question))
            (name)
          },
        }
      }
    }
  }
}

pub fn TreeEntry(context: &TreeEntryContext) -> ::maud::Markup {
  html! {
    div {
      @match context.entry.kind() {
        Some(ObjectType::Tree) => (Tree(context.entry.as_tree().unwrap(), context)),
        Some(ObjectType::Blob) => (Blob(context.entry.as_blob().unwrap(), context)),
        Some(ObjectType::Tag) => "Can't render ObjectType::Tag yet",
        Some(ObjectType::Commit) => "Can't render ObjectType::Commit yet",
        Some(ObjectType::Any) => "Can't render ObjectType::Any yet",
        None => "Can't render without an ObjectType",
      }
    }
  }
}

pub fn Tree(tree: &git2::Tree, context: &TreeEntryContext) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h2 {
          (FAM::FixedWidth(FA::File)) " "
          span.path (Components(context))
          " at "
          (super::Reference(&context.commit))
        }
      }
      div.block-details {
        ul.fa-ul {
          @if let Some(parent) = context.parent() {
            li { (FAM::Li(FA::LevelUp)) a href={ "/" (context.repo_path) "/tree/" (context.reff) (parent) } { ".." } }
          }
          @for entry in tree.iter().collect::<Vec<_>>().tap(|v| v.sort_by_key(|e| Sorter(e.kind()))) {
            (TreeEntryStub(context, &entry))
          }
        }
      }
    }
  }
}

pub fn Blob(blob: &git2::Blob, context: &TreeEntryContext) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h2 {
          (FAM::FixedWidth(FA::File)) " "
          span.path (Components(context))
          " at "
          (super::Reference(&context.commit))
        }
      }
      pre.block-details {
        @match blob.is_binary() {
          true => code { "Binary file" },
          false => code class={ "hljs lang-" (context.extension().unwrap_or("")) } {
            @for (i, line) in str::from_utf8(blob.content()).unwrap().lines().enumerate() {
              div.line {
                a.line-num id={ "L" (i + 1) } href={ "#L" (i + 1) } data-line-num=(format!("{: >4}", i + 1)) { " " }
                span.text (line)
              }
            }
          },
        }
      }
      (super::HighlightJS())
    }
  }
}

pub struct Components<'a>(pub &'a TreeEntryContext<'a>);

impl<'a> Render for Components<'a> {
  #[allow(cyclomatic_complexity)]
  fn render_to(&self, buffer: &mut String) {
    let context = self.0;
    let is_blob = context.entry.kind() == Some(ObjectType::Blob);
    buffer.push_str(&html!({ a.path-component href={ "/" (context.repo_path) "/tree/" (context.reff) } (context.repo_path) }).into_string());
    let mut parent = "/".to_owned();
    let components: Vec<_> = context.entry_path.split_terminator('/').collect();
    for (i, component) in components.iter().enumerate() {
      if *component == "" {
        continue;
      } else if is_blob && i == components.len() - 1 {
        buffer.push_str(&html!({ "/" a.path-component href={ "/" (context.repo_path) "/blob/" (context.reff) (parent) (component) } (component) }).into_string());
      } else {
        buffer.push_str(&html!({ "/" a.path-component href={ "/" (context.repo_path) "/tree/" (context.reff) (parent) (component) } (component) }).into_string());
        parent.push_str(component);
        parent.push('/');
      }
    }
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

// impl<'a> super::repository_wrapper::RepositoryTab for &'a TreeEntry<'a> {
//   fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Files) }
// }
// 
// impl<'a> super::repository_wrapper::RepositoryTab for &'a Tree<'a> {
//   fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Files) }
// }
// 
// impl<'a> super::repository_wrapper::RepositoryTab for &'a Blob<'a> {
//   fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Files) }
// }
