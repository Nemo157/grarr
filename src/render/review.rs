use git_appraise::{ Review };
use super::{ RequestRenderer, CIStatusesRenderer, AnalysesRenderer, CommentsRenderer };

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
    #(CIStatusesRenderer(review.ci_statuses()))
    #(AnalysesRenderer(review.analyses()))
    #(CommentsRenderer(review.comments()))
  }
}
