use git_appraise::{ Review, Status };
use super::{ RequestRenderer, CommentRenderer };

renderers! {
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
