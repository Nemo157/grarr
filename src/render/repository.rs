use std::str;
use git2::{ Repository };
use pulldown_cmark::{ Parser, html, Event, Tag };
use maud::PreEscaped;
use maud_pulldown_cmark::markdown;

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

  RepositoryStubRenderer(name: &'a str, repo: &'a Repository) {
    li class="repo-stub" {
      i class="fa fa-git-square fa-li" { } " "
      a href=#name {
        #name
      }
      #if let Some(description) = description(repo) {
        blockquote {
          #(PreEscaped(description))
        }
      }
    }
  }

  RepositoriesRenderer(repos: &'a Vec<(String, Repository)>) {
    ul class="fa-ul" {
      #for &(ref path, ref repo) in repos {
        #RepositoryStubRenderer(&*path, repo)
      }
    }
  }
}
