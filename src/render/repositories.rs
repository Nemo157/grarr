use std::fmt;

use ammonia;
use git2;
use pulldown_cmark::{ Parser, html, Event, Tag };

use repository_extension::RepositoryExtension;
use super::{utils, repository};
use super::fa::{FAM, FA};

fn description(repo: &git2::Repository) -> Option<String> {
    let head_id = expect!(try_expect!(try_expect!(repo.head()).resolve()).target());
    // Render the readme and grab the first <p> element from it.
    repository::find_readme(head_id, repo)
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
            ammonia::clean(&unsafe_html)
        })
}

fn repo_stub((path, repo): (String, git2::Repository)) -> impl fmt::Display {
    fmt!(r#"
        <div class="block">
            <div class="block-header">
                <div class="row center">
                    {icon}
                    <div class="column">
                        <h3><a href="/{path}">{path}</a></h3>
                        {fork}
                    </div>
                </div>
            </div>
            {description}
        </div>
    "#,
    path = path,
    icon = {
        if let Some(_) = repo.origin_url() {
            FAM::X(2, FA::CodeFork)
        } else {
            FAM::X(2, FA::Home)
        }
    },
    fork = {
        if let Some(mut origin) = repo.origin_url() {
            if origin.starts_with("https://") {
                let text = origin.split_off("https://".len());
                format!("<h6>(fork of <a href=\"{0}{1}\">{1}</a>)</h6>", origin, text)
            } else {
                format!("<h6>(fork of {})</h6>", origin)
            }
        } else {
            String::new()
        }
    },
    description = {
        if let Some(desc) = description(&repo) {
            format!(r#"<div class="block-details">{0}</div>"#, desc)
        } else {
            String::new()
        }
    })
}

pub fn repositories(repos: Vec<(String, git2::Repository)>) -> impl fmt::Display {
    utils::joined(repos.into_iter().map(repo_stub))
}
