use std::fmt;
use ammonia;
use maud::Render;
use pulldown_cmark::{ Parser, html };
use take::Take;

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

struct Joined<D, I>(Take<I>) where D: fmt::Display, I: Iterator<Item=D>;

pub fn joined<D, I>(iter: I) -> impl fmt::Display where D: fmt::Display, I: Iterator<Item=D> {
    Joined(Take::new(iter))
}

impl<D, I> fmt::Display for Joined<D, I> where D: fmt::Display, I: Iterator<Item=D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in self.0.take() {
            d.fmt(f)?;
        }
        Ok(())
    }
}
