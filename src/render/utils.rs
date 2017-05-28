use std::fmt;
use ammonia;
use maud::Render;
use pulldown_cmark::{ Parser, html };

pub fn MaybeLink(href: &str, text: &str) -> ::maud::Markup {
    html! {
        @if href.starts_with("http") {
            a href=(href) { (text) }
        } @else {
            (text)
        }
    }
}

pub struct Markdown<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render_to(&self, buffer: &mut String) {
        let mut unsafe_html = String::new();
        let parser = Parser::new(self.0.as_ref());
        html::push_html(&mut unsafe_html, parser);
        buffer.push_str(&ammonia::clean(&unsafe_html));
    }
}

pub fn markdown(s: &str) -> impl fmt::Display {
    let mut unsafe_html = String::new();
    let parser = Parser::new(s);
    html::push_html(&mut unsafe_html, parser);
    ammonia::clean(&unsafe_html)
}
