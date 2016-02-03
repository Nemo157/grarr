use git_appraise::{ Event, Events };
use super::{ RequestRenderer, CIStatusRenderer, AnalysisRenderer, CommentRenderer };

renderers! {
  EventsRenderer(events: Events) {
    @for event in events {
      ^EventRenderer(event)
    }
  }

  EventRenderer(event: Box<Event>) {
    @if let Some(request) = event.as_request() {
      ^RequestRenderer(request)
    }
    @if let Some(comment) = event.as_comment() {
      ^CommentRenderer(comment)
    }
    @if let Some(analysis) = event.as_analysis() {
      ^AnalysisRenderer(analysis)
    }
    @if let Some(ci_status) = event.as_ci_status() {
      ^CIStatusRenderer(ci_status)
    }
  }
}
