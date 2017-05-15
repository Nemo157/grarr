use git_appraise;

pub fn Events(root: String, events: git_appraise::Events) -> ::maud::Markup {
    html! {
        @for event in events {
            (Event(root.clone(), event))
        }
    }
}

pub fn Event(root: String, event: Box<git_appraise::Event>) -> ::maud::Markup {
    html! {
        @if let Some(request) = event.as_request() {
            (super::Request(&root, request))
        }
        @if let Some(comment) = event.as_comment() {
            (super::Comment(comment))
        }
        @if let Some(analysis) = event.as_analysis() {
            (super::Analysis(analysis))
        }
        @if let Some(ci_status) = event.as_ci_status() {
            (super::CIStatus(ci_status))
        }
    }
}
