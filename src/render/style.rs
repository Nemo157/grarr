use settings::Settings;

renderers! {
  Style(settings: &'a Settings) {
    link rel="stylesheet" href="/-/static/css/font-awesome.min.css" { }
    link rel="stylesheet" href="/-/static/css/layout.css" { }
    link rel="stylesheet" href=^(format!("/-/static/css/theme-{}.css", settings.theme)) { }
  }
}
