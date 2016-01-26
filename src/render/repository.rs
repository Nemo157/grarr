use std::str;
use git2::{ Repository };
use pulldown_cmark::{ Parser, html, Event, Tag };
use maud::PreEscaped;
use maud_pulldown_cmark::markdown;
use repository_tree::RepositoryTreeEntry;

fn find_readme(repo: &Repository) -> Option<String> {
  let head_id = expect!(try_expect!(try_expect!(repo.head()).resolve()).target());
  let head = try_expect!(repo.find_commit(head_id));
  let tree = try_expect!(head.tree());
  let entry = expect!(tree.get_name("README").or_else(|| tree.get_name("README.md")));
  let object = try_expect!(entry.to_object(repo));
  let blob = expect!(object.as_blob());
  str::from_utf8(blob.content()).ok().map(|s| s.to_string())
}

fn description(repo: &Repository) -> Option<String> {
  // Render the readme and grab the first <p> element from it.
  find_readme(repo)
    .map(|readme| {
      let mut s = String::new();
      html::push_html(
        &mut s,
        Parser::new(&*readme)
          .skip_while(|ev| match ev {
            &Event::Start(Tag::Paragraph) => false,
            _ => true,
          })
          .take_while(|ev| match ev {
            &Event::End(Tag::Paragraph) => false,
            _ => true,
          }));
      s
    })
}

renderers! {
  RepositoryRenderer(name: &'a str, actual: &'a str, repo: &'a Repository) {
    h1 {
      i class="fa fa-git-square" { } " "
      a href={ "/" #name }  { #name }
      #if name != actual {
        " "
        small {
          "(alias of " a href={ "/" #actual } { #actual } ")"
        }
      }
    }
    #if let Some(readme) = find_readme(repo) {
      #(markdown::from_string(&*readme))
    }
  }

  RepositoryStubRenderer(path: &'a str, name: &'a str, repo: &'a Repository) {
    li class="repo-stub" {
      i class="fa fa-git-square fa-li" { } " "
      a href={ #path "/" #name } {
        #name
      }
      #if let Some(description) = description(repo) {
        blockquote {
          #(PreEscaped(description))
        }
      }
    }
  }

  RepositoriesRenderer(path: &'a str, repos: &'a Vec<RepositoryTreeEntry>) {
    ul class="fa-ul" {
      #for entry in repos {
        #if let &RepositoryTreeEntry::Dir(ref name, ref repos) = entry {
          li {
            i class="fa fa-sitemap fa-li" { } " "
            #name
            #RepositoriesRenderer(&*(path.to_string() + "/" + name), repos)
          }
        }
        #if let &RepositoryTreeEntry::Alias(ref alias, ref actual) = entry {
          li {
            i class="fa fa-tag fa-li" { } " "
            a href={ #path "/" #alias } {
              #alias
            }
            " alias of "
            a href=#actual {
              #actual
            }
          }
        }
        #if let &RepositoryTreeEntry::Repo(ref name, ref repo) = entry {
          #RepositoryStubRenderer(path, name, repo)
        }
      }
    }
  }
}
