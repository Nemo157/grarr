use git_appraise;

renderers! {
  Reviews(root: &'a str, reviews: &'a Vec<git_appraise::Review<'a>>) {
    @for review in reviews {
      ^ReviewStub(root, review)
    }
  }

  ReviewStub(root: &'a str, review: &'a git_appraise::Review<'a>) {
    ^super::RequestStub(root, review.request())
  }

  Review(root: &'a str, review: &'a git_appraise::Review<'a>) {
    div.review {
      ^super::Events(root.to_owned(), review.events())
    }
  }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Review<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Reviews }
}

impl<'a> super::repository_wrapper::RepositoryTab for &'a Reviews<'a> {
  fn tab() -> super::repository_wrapper::Tab { super::repository_wrapper::Tab::Reviews }
}
