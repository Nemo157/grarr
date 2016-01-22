use git_appraise::{ Review };
use super::{ RequestRenderer, RequestStubRenderer, CIStatusesRenderer, AnalysesRenderer, CommentsRenderer };

renderers! {
  ReviewsRenderer(reviews: &'a Vec<Review<'a>>) {
    #for review in reviews {
      #ReviewStubRenderer(review)
    }
  }

  ReviewStubRenderer(review: &'a Review<'a>) {
    div class="review-stub" {
      #RequestStubRenderer(&review.id(), review.request())
    }
  }

  ReviewRenderer(review: &'a Review<'a>) {
    div class="review" {
      #RequestRenderer(&review.id(), review.request())
      #CIStatusesRenderer(review.ci_statuses())
      #AnalysesRenderer(review.analyses())
      #CommentsRenderer(review.comments())
    }
  }
}
