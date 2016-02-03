renderers! {
  Avatar(email: &'a str) {
    img.avatar width=30 height=30 src={ "/-/avatars/" ^email } {}
  }
}
