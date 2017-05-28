#[macro_export]
macro_rules! to_string {
    ($($x:tt)*) => {{
        html!($($x)*).into_string()
    }}
}

#[macro_export]
macro_rules! html2 {
    ($writer:expr, { $($x:tt)* }) => {{
        write!($writer, "{}", to_string!({ $($x)* }))
    }};
}

#[macro_export]
macro_rules! fmt {
    ($fmt:expr, $($args:tt)*) => {
        $crate::render::MovableArguments(::take::Take::new(move |f: &mut fmt::Formatter| { write!(f, $fmt, $($args)*) }))
    };
}
