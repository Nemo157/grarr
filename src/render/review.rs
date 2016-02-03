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

impl<'a> super::repository_wrapper::RepositoryTab for &'a Review<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Reviews }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Reviews<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Reviews }
}
