#[macro_export]
macro_rules! to_string {
  ($($x:tt)*) => {{
    let mut s = String::new();
    html!(s, $($x)*).unwrap();
    s
  }}
}

#[macro_export]
macro_rules! renderers {
  () => ();
  ($name:ident { $($html:tt)* } $($rest:tt)*) => (
    pub struct $name;
    impl ::maud::Render for $name {
      fn render(&self, mut w: &mut ::std::fmt::Write) -> ::std::fmt::Result {
        html!(w, $($html)*)
      }
    }
    renderers!($($rest)*);
  );
  ($name:ident ($($var:ident : $var_type:ty),+) { $($html:tt)* } $($rest:tt)*) => (
    pub struct $name<'a>($(pub $var_type),+);
    impl<'a> ::maud::Render for $name<'a> {
      fn render(&self, mut w: &mut ::std::fmt::Write) -> ::std::fmt::Result {
        let &$name($($var)+) = self;
        html!(w, $($html)*)
      }
    }
    renderers!($($rest)*);
  );
}
