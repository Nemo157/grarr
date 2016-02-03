use git2;
use maud::PreEscaped;

renderers! {
  Signature(signature: &'a git2::Signature<'a>) {
    @if let Some(email) = signature.email() {
      ^super::Avatar(email)
    }
    @if let Some(name) = signature.name() {
      span.name ^name
      ^PreEscaped("&nbsp;")
    }
    @if let Some(email) = signature.email() {
      a href={ "mailto:" ^email } span.email ^email
      ^PreEscaped("&nbsp;")
    }
  }
}
