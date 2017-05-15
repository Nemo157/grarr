pub fn Avatar(email: &str, name: &Option<&str>) -> ::maud::Markup {
  html! {
    img.avatar
      width="30"
      height="30"
      alt={ @if let Some(name) = *name { (name) " " } "<" (email) ">" }
      title={ @if let Some(name) = *name { (name) " " } "<" (email) ">" }
      src={ "/-/avatars/" (email) }
      {}
  }
}
