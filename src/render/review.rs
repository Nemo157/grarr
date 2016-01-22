use git_appraise::{ Review };
use super::{ RequestStubRenderer, EventsRenderer };

renderers! {
  ReviewsRenderer(reviews: &'a Vec<Review<'a>>) {
    #for review in reviews {
      #ReviewStubRenderer(review)
    }
  }

  ReviewStubRenderer(review: &'a Review<'a>) {
    div class="review-stub" {
      #RequestStubRenderer(review.request())
    }
  }

  ReviewRenderer(review: &'a Review<'a>) {
    div class="review" {
      #EventsRenderer(review.events())
    }
  }
}
