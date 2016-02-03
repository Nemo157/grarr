use git_appraise;

renderers! {
  Reviews(reviews: &'a Vec<git_appraise::Review<'a>>) {
    @for review in reviews {
      ^ReviewStub(review)
    }
  }

  ReviewStub(review: &'a git_appraise::Review<'a>) {
    ^super::RequestStub(review.request())
  }

  Review(review: &'a git_appraise::Review<'a>) {
    .review {
      ^super::Events(review.events())
    }
  }
}
