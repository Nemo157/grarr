use git2;
use maud::PreEscaped;

pub fn Signature(signature: git2::Signature, include_avatar: bool) -> ::maud::Markup {
    html! {
        @if include_avatar {
            @if let Some(email) = signature.email() {
                (super::Avatar(email, &signature.name()))
            }
        }
        @if let Some(name) = signature.name() {
            @if let Some(email) = signature.email() {
                a.user href={ "mailto:" (email) } title={ "<" (email) ">" } (name)
            } @else {
                span.user (name)
            }
            (PreEscaped("&nbsp;"))
        }
    }
}
