use git_appraise::{ Review };
use super::{ RequestStubRenderer, EventsRenderer };

renderers! {
  ReviewsRenderer(name: &'a str, actual: &'a str, reviews: &'a Vec<Review<'a>>) {
    h1 {
      i class="fa fa-git-square" { } " "
      a href={ "/" #name }  { #name }
      #if name != actual {
        " "
        small {
          "(alias of " a href={ "/" #actual } { #actual } ")"
        }
      }
    }
    div class="repository" {
      div class="options" {
        div class="overview" { a href={ "/" #name } { "Overview" } }
        div class="commits" { a href={ "/" #name "/commits" } { "Commits" } }
        div class="selected reviews" { a href={ "/" #name "/reviews" } { "Reviews" } }
      }
      div class="content reviews" {
        #for review in reviews {
          #ReviewStubRenderer(review)
        }
      }
    }
  }

  ReviewStubRenderer(review: &'a Review<'a>) {
    #RequestStubRenderer(review.request())
  }

  ReviewRenderer(review: &'a Review<'a>) {
    div class="review" {
      #EventsRenderer(review.events())
    }
  }
}
