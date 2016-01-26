use maud::PreEscaped;

renderers! {
  Style {
    link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/font-awesome/4.5.0/css/font-awesome.min.css" { }
    style type="text/css" {
      #PreEscaped(include_str!("style.css"))
    }
  }
}
