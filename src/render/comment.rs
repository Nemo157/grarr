use git_appraise;
use super::utils::Markdown;
use chrono::naive::datetime::NaiveDateTime;

pub fn CommentHeader(comment: &git_appraise::Comment) -> ::maud::Markup {
    html! {
        div.block-header.row.center {
            (super::Avatar(comment.author().unwrap_or("unknown@example.org"), &None))
            div.column {
                div {
                    span.user
                        (comment.author().unwrap_or("<unknown author>"))
                    " commented "
                    @if let Some(timestamp) = comment.timestamp() {
                        "on "
                        span.timestamp
                            (NaiveDateTime::from_timestamp(timestamp.seconds(), 0))
                        " "
                    }
                    "with status "
                    span class={
                        "resolved "
                        @match comment.resolved() {
                            Some(true) => "lgtm",
                            Some(false) => "nmw",
                            None => "fyi",
                        }
                    } {
                        @match comment.resolved() {
                            Some(true) => "👍",
                            Some(false) => "👎",
                            None => "ℹ️",
                        }
                    }
                }
            }
        }
    }
}

pub fn CommentDetails(comment: &git_appraise::Comment) -> ::maud::Markup {
    html! {
        @if let Some(location) = comment.location() {
            div.block-details.comment-details {
                pre { code { ((format!("{:?}", location))) } }
            }
        }
        @if let Some(description) = comment.description() {
            div.block-details.comment-details {
                (Markdown(description))
            }
        }
    }
}

pub fn Comment(comment: &git_appraise::Comment) -> ::maud::Markup {
    html! {
        div.block.comment {
            (CommentHeader(comment))
            (CommentDetails(comment))
        }
    }
}
