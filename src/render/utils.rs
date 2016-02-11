renderers! {
  MaybeLink(href: &'a str, text: &'a str) {
    @if href.starts_with("http") {
      a href=^href { ^text }
    } @else {
      ^text
    }
  }
}
