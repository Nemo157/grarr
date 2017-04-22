use std::str;
use ammonia;
use git2::{ self, Oid };
use pulldown_cmark::{ Parser, html, Event, Tag };
use maud::{ PreEscaped };
use super::utils::Markdown;
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

fn description(repo: &git2::Repository) -> Option<PreEscaped<String>> {
  let head_id = expect!(try_expect!(try_expect!(repo.head()).resolve()).target());
  // Render the readme and grab the first <p> element from it.
  find_readme(head_id, repo)
    .map(|readme| {
      let mut unsafe_html = String::new();
      html::push_html(
        &mut unsafe_html,
        Parser::new(&*readme)
          .skip_while(|ev| match *ev {
            Event::Start(Tag::Paragraph) => false,
            _ => true,
          })
          .take_while(|ev| match *ev {
            Event::End(Tag::Paragraph) => false,
            _ => true,
          }));
      let safe_html = ammonia::clean(&unsafe_html);
      PreEscaped(safe_html)
    })
}

pub fn Repository(repo: &git2::Repository, head_id: &Oid) -> ::maud::Markup {
  html! {
    @if let Some(readme) = find_readme(*head_id, repo) {
      div.block {
        div.block-details {
          (Markdown(&*readme))
        }
      }
    }
  }
}

pub fn RepositoryIcon(mul: &u8, repo: &git2::Repository) -> ::maud::Markup {
  html! {
    @match repo.origin_url() {
      Some(_) => (FAM::X(*mul, FA::CodeFork)),
      None => (FAM::X(*mul, FA::Home)),
    }
  }
}

pub fn RepositoryHeader(path: &str, repo: &git2::Repository) -> ::maud::Markup {
  html! {
    div.block-header {
      div.row.center {
        (RepositoryIcon(&3, repo))
        div.column {
          h1 { a href={ "/" (path) } { (path) } }
          @if let Some(origin) = repo.origin_url() {
            h4 { "(fork of " (super::MaybeLink(&origin, &origin)) ")" }
          }
          @if let Some(mirrors) = repo.mirrors() {
            h4 {
              "(mirrored on"
              @for (name, url) in mirrors {
                " " a href=(url) { (name) }
              }
              ")"
            }
          }
        }
        @if repo.find_branch("gh-pages", git2::BranchType::Local).is_ok() {
          div.column.fixed {
            h3 {
              a href={ "/" (path) "/pages/" } {
                (FAM::Lg(FA::Book))
                " Pages"
              }
            }
          }
        }
      }
    }
  }
}

pub fn RepositoryStub(path: &str, repo: &git2::Repository) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        div.row.center {
          (RepositoryIcon(&2, repo))
          div.column {
            h3 { a href={ "/" (path) } { (path) } }
            @if let Some(origin) = repo.origin_url() {
              h6 { "(fork of " (super::MaybeLink(&origin, &origin)) ")" }
            }
          }
        }
      }
      @if let Some(description) = description(repo) {
        div.block-details {
          (description)
        }
      }
    }
  }
}

pub fn Repositories(repos: Vec<(String, git2::Repository)>) -> ::maud::Markup {
  html! {
    @for (path, repo) in repos {
      (RepositoryStub(&path, &repo))
    }
  }
}

// impl<'a> super::repository_wrapper::RepositoryTab for &'a Repository<'a> {
//   fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Overview) }
// }
