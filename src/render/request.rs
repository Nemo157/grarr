use git_appraise;
use git2::Oid;
use super::utils::Markdown;
use chrono::naive::datetime::NaiveDateTime;

fn summary(request: &git_appraise::Request) -> Option<&str> {
    request.description()
        .and_then(|description| description.lines().nth(0))
}

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: Oid) -> String {
    oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

pub fn Request(root: &str, request: &git_appraise::Request) -> ::maud::Markup {
    html! {
        div.block.request {
            (RequestHeader(root, request))
            (RequestDetails(request))
        }
    }
}

pub fn RequestStub(root: &str, request: &git_appraise::Request) -> ::maud::Markup {
    html! {
        div.block.request {
            (RequestHeader(root, request))
        }
    }
}

pub fn RequestHeader(root: &str, request: &git_appraise::Request) -> ::maud::Markup {
    html! {
        div.block-header {
            div.row {
                (super::Avatar(request.requester().unwrap_or("unknown@example.org"), &None))
                div.column {
                    div {
                        a href={ (root) "/review/" (request.commit_id()) } {
                            span.id (short(request.commit_id()))
                            " "
                            @match summary(request) {
                                Some(summary) => (summary),
                                None => "<No summary provided>",
                            }
                        }
                    }
                    small {
                        span.user
                            (request.requester().unwrap_or("<unknown requester>"))
                        " wants to merge "
                        span.ref
                            (request.review_ref().unwrap_or("<unknown ref>"))
                        " into "
                        span.ref
                            (request.target_ref().unwrap_or("<unknown ref>"))
                    }
                }
                div.column.fixed {
                    @if let Some(timestamp) = request.timestamp() {
                        small.timestamp
                            (NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
                    }
                }
            }
        }
    }
}

pub fn RequestDetails(request: &git_appraise::Request) -> ::maud::Markup {
    html! {
        div.block-details.request-details {
            @match request.description() {
                Some(description) => (Markdown(description)),
                None => i "No description provided",
            }
        }
    }
}
