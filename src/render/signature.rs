use git2;
use maud::PreEscaped;

renderers! {
  Signature(signature: &'a git2::Signature<'a>, include_avatar: &'a bool) {
    @if *include_avatar {
      @if let Some(email) = signature.email() {
        ^super::Avatar(email, &signature.name())
      }
    }
    @if let Some(name) = signature.name() {
      @if let Some(email) = signature.email() {
        a.user href={ "mailto:" ^email } title={ "<" ^email ">" } ^name
      } @else {
        span.user ^name
      }
      ^PreEscaped("&nbsp;")
    }
  }
}
