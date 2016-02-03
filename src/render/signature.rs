use git2::Signature as GitSignature;
use maud::PreEscaped;
use super::Avatar;

renderers! {
  Signature(signature: &'a GitSignature<'a>) {
    @if let Some(email) = signature.email() {
      ^Avatar(email)
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
