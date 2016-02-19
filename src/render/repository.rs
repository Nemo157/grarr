use std::str;
use git2::{ self, Oid };
use pulldown_cmark::{ Parser, html, Event, Tag };
use maud::{ PreEscaped };
use maud_pulldown_cmark::Markdown;
use super::fa::{ FA, FAM };
use { RepositoryExtension };

fn find_readme(head_id: Oid, repo: &git2::Repository) -> Option<String> {
  let head = try_expect!(repo.find_commit(head_id));
  let tree = try_expect!(head.tree());
  let entry = expect!(tree.get_name("README").or_else(|| tree.get_name("README.md")));
  let object = try_expect!(entry.to_object(repo));
  let blob = expect!(object.as_blob());
  str::from_utf8(blob.content()).ok().map(|s| s.to_owned())
}

fn description(repo: &git2::Repository) -> Option<String> {
  let head_id = expect!(try_expect!(try_expect!(repo.head()).resolve()).target());
  // Render the readme and grab the first <p> element from it.
  find_readme(head_id, repo)
    .map(|readme| {
      let mut s = String::new();
      html::push_html(
        &mut s,
        Parser::new(&*readme)
          .skip_while(|ev| match *ev {
            Event::Start(Tag::Paragraph) => false,
            _ => true,
          })
          .take_while(|ev| match *ev {
            Event::End(Tag::Paragraph) => false,
            _ => true,
          }));
      s
    })
}

renderers! {
  Repository(repo: &'a git2::Repository, head_id: &'a Oid) {
    @if let Some(readme) = find_readme(*head_id, repo) {
      div.block {
        div.block-details {
          ^Markdown::from_string(&*readme)
        }
      }
    }
  }

  RepositoryIcon(mul: &'a u8, repo: &'a git2::Repository) {
    @match repo.origin_url() {
      Some(_) => ^FAM::X(*mul, FA::CodeFork),
      None => ^FAM::X(*mul, FA::Home),
    }
  }

  RepositoryHeader(path: &'a str, repo: &'a git2::Repository) {
    div.block-header {
      div.row.center {
        ^RepositoryIcon(&3, repo)
        div.column {
          h1 { a href={ "/" ^path } { ^path } }
          @if let Some(origin) = repo.origin_url() {
            h4 { "(fork of " ^super::MaybeLink(&origin, &origin) ")" }
          }
        }
      }
    }
  }

  RepositoryStub(path: &'a str, repo: &'a git2::Repository) {
    div.block {
      div.block-header {
        div.row.center {
          ^RepositoryIcon(&2, repo)
          div.column {
            h3 { a href={ "/" ^path } { ^path } }
            @if let Some(origin) = repo.origin_url() {
              h6 { "(fork of " ^super::MaybeLink(&origin, &origin) ")" }
            }
          }
        }
      }
      @if let Some(description) = description(repo) {
        div.block-details {
          ^PreEscaped(description)
        }
      }
    }
  }

  Repositories(repos: Vec<(String, git2::Repository)>) {
    ^RepositoriesHeader
    @for (path, repo) in repos {
      ^RepositoryStub(&path, &repo)
    }
  }

  RepositoriesHeader {
    div.block {
      div.block-header {
        h1 { a href="/" { "Repositories" } }
      }
    }
  }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Repository<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Overview }
}
