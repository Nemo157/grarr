#[macro_export]
macro_rules! to_string {
    ($($x:tt)*) => {{
        html!($($x)*).into_string();
    }}
}
