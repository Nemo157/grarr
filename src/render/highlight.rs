pub fn HighlightJS() -> ::maud::Markup {
  html! {
    link rel="stylesheet" href="/-/static/css/highlight-solarized-light.css" {}
    script src="/-/static/js/highlight.js" {}
    script { "hljs.initHighlightingOnLoad()" }
  }
}
