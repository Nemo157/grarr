use std::str;
use git2::{ self, Oid };
use super::utils::Markdown;
use super::fa::{ FA, FAM };
use { RepositoryExtension };

pub fn find_readme(head_id: Oid, repo: &git2::Repository) -> Option<String> {
    let head = try_expect!(repo.find_commit(head_id));
    let tree = try_expect!(head.tree());
    let entry = expect!(tree.get_name("README").or_else(|| tree.get_name("README.md")));
    let object = try_expect!(entry.to_object(repo));
    let blob = expect!(object.as_blob());
    str::from_utf8(blob.content()).ok().map(|s| s.to_owned())
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
