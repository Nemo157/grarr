use git_appraise;

pub fn Reviews(root: &str, reviews: &Vec<git_appraise::Review>) -> ::maud::Markup {
    html! {
        @for review in reviews {
            (ReviewStub(root, review))
        }
    }
}

pub fn ReviewStub(root: &str, review: &git_appraise::Review) -> ::maud::Markup {
    html! {
        (super::RequestStub(root, review.request()))
    }
}

pub fn Review(root: &str, review: &git_appraise::Review) -> ::maud::Markup {
    html! {
        div.review {
            (super::Events(root.to_owned(), review.events()))
        }
    }
}

// impl<'a> super::repository_wrapper::RepositoryTab for &'a Review<'a> {
//     fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Reviews) }
// }
// 
// impl<'a> super::repository_wrapper::RepositoryTab for &'a Reviews<'a> {
//     fn tab() -> Option<super::repository_wrapper::Tab> { Some(super::repository_wrapper::Tab::Reviews) }
// }
