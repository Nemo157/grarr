use maud::PreEscaped;

renderers! {
  Style {
    style type="text/css" {
      #PreEscaped(include_str!("style.css"))
    }
  }
}
