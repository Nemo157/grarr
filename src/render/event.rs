use git_appraise;

renderers! {
  Events(events: git_appraise::Events) {
    @for event in events {
      ^Event(event)
    }
  }

  Event(event: Box<git_appraise::Event>) {
    @if let Some(request) = event.as_request() {
      ^super::Request(request)
    }
    @if let Some(comment) = event.as_comment() {
      ^super::Comment(comment)
    }
    @if let Some(analysis) = event.as_analysis() {
      ^super::Analysis(analysis)
    }
    @if let Some(ci_status) = event.as_ci_status() {
      ^super::CIStatus(ci_status)
    }
  }
}
