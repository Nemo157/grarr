use chrono;
use git_appraise;

#[macro_use]
mod macros;

use std::fmt;
use maud::{ PreEscaped, Render };
use git_appraise::{ Review, Status };
use maud_pulldown_cmark::markdown;

renderers! {
  Style {
    style type="text/css" {
      #PreEscaped(include_str!("style.css"))
    }
  }

  RequestRenderer(request: &'a git_appraise::Request) {
    div {
      #if let Some(requester) = request.requester() {
        div { "Requester: " #requester }
      }
      #if let Some(timestamp) = request.timestamp() {
        div { "Timestamp: " #(chrono::naive::datetime::NaiveDateTime::from_timestamp(timestamp.seconds(), 0)) }
      }
      #if let (Some(review_ref), Some(target_ref)) = (request.review_ref(), request.target_ref()) {
        div { "Proposed merge: " #review_ref " -> " #target_ref }
      }
      #if let Some(reviewers) = request.reviewers() {
        div { "Reviewers:"
          ul {
            #for reviewer in reviewers {
              li #reviewer
            }
          }
        }
      }
      #if let Some(ref description) = request.description() {
        div {
          "Description: "
          div { #(markdown::from_string(description)) }
        }
      }
    }
  }

  CommentRenderer(comment: &'a git_appraise::Comment) {
    div {
      #if let Some(author) = comment.author() {
        div { "Comment from " #author }
      }
      div { "Comment Status: " #comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi") }
      #if let Some(location) = comment.location() {
        div { "Referencing " #(format!("{:?}", location)) }
      }
      #if let Some(description) = comment.description() {
        div { #(markdown::from_string(description)) }
      }
    }
  }

  ReviewsRenderer(reviews: &'a Vec<Review<'a>>) {
    ol {
      #for review in reviews {
        li {
          a href={ "/" #review.id() } #review.id()
          " -> "
          #review.request().description().unwrap()
        }
      }
    }
  }

  ReviewRenderer(review: &'a Review<'a>) {
    #(RequestRenderer(review.request()))
    div {
      "CI Statuses: "
      ul {
        #for status in review.ci_statuses() {
          li {
            #if let Some(url) = status.url() {
              a href={ #url } #status.agent().unwrap_or("<Unknown agent>")
            }
            #if status.url().is_none() {
              #status.agent().unwrap_or("<Unknown agent>")
            }
            ": "
            #status.status().map(|s| match s { Status::Success => "success", Status::Failure => "failure" }).unwrap_or("null")
          }
        }
      }
    }
    div {
      "Analyses: "
      ul {
        #for analysis in review.analyses() {
          #if let Some(url) = analysis.url() {
            li {
              a href={ #url } #url
            }
          }
        }
      }
    }
    div {
      "Comments: "
      ul {
        #for comment in review.comments() {
          li {
            #(CommentRenderer(&comment))
          }
        }
      }
    }
  }

}

pub struct Wrapper<T: Render>(pub T);

impl<T: Render> Render for Wrapper<T> {
  fn render(&self, mut w: &mut fmt::Write) -> fmt::Result {
    html!(w, {
      html {
        head {
          #Style
        }
        body {
          #(self.0)
        }
      }
    })
  }
}
