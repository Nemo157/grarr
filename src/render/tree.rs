use std::str;
use std::fmt;
use super::fa::{ FA };
use git2::{ Tree, TreeEntry, ObjectType, Repository, Blob };
use std::path::{ Path, PathBuf, Components, Component };

renderers! {
  TreeEntryStubRenderer(parent: &'a str, entry: &'a TreeEntry<'a>) {
    #if let Some(name) = entry.name() {
      div {
        a href={ #parent "/" #name } {
          #{match entry.kind() {
            Some(ObjectType::Tree) => FA::Sitemap,
            Some(ObjectType::Blob) => FA::File,
            _ => FA::Question,
          } }
          " "
          #name
        }
      }
    }
  }

  TreeEntryRenderer(repo: &'a Repository, parent: &'a str, path: &'a Path, entry: &'a TreeEntry<'a>) {
    div {
      #if let Some(ObjectType::Tree) = entry.kind() {
        #TreeRenderer(parent, path, entry.to_object(repo).unwrap().as_tree().unwrap())
      }
      #if let Some(ObjectType::Blob) = entry.kind() {
        #BlobRenderer(parent, path, entry.to_object(repo).unwrap().as_blob().unwrap())
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

  RootTreeRenderer(parent: &'a str, tree: &'a Tree<'a>) {
    h2 class="path" { #ComponentsRenderer(parent, PathBuf::from("/").components()) }
    #for entry in tree.iter() {
      #TreeEntryStubRenderer(parent, &entry)
    }
  }

  TreeRenderer(parent: &'a str, path: &'a Path, tree: &'a Tree<'a>) {
    h2 class="path" { #ComponentsRenderer(parent, path.components()) }
    #for entry in tree.iter() {
      #TreeEntryStubRenderer(&(parent.to_string() + path.to_str().unwrap()), &entry)
    }
  }

  BlobRenderer(parent: &'a str, path: &'a Path, blob: &'a Blob<'a>) {
    h2 class="path" { #ComponentsRenderer(parent, path.components()) }
    #if blob.is_binary() {
      pre { code { "Binary file" } }
    }
    #if !blob.is_binary() {
      pre { code { #(str::from_utf8(blob.content()).unwrap()) } }
    }
  }
}

pub struct ComponentsRenderer<'a>(&'a str, pub Components<'a>);

impl<'a> ::maud::RenderOnce for ComponentsRenderer<'a> {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    let mut parent = self.0.to_string();
    for component in self.1 {
      match component {
        Component::RootDir => {
          try!(html!(w, { a class="path-component" href={ #parent } "<root>" }));
        },
        Component::Normal(component) => {
          try!(html!(w, { "/" a class="path-component" href={ #parent "/" #component.to_str().unwrap() } #component.to_str().unwrap() }));
          parent = parent + "/" + component.to_str().unwrap();
        },
        _ => {
        },
      }
    }
    Ok(())
  }
}
