use std::fmt;

use iron;
use iron::status::Status;

pub fn error((status, error): (Status, Box<iron::Error + Send>)) -> impl fmt::Display {
    fmt!(r#"
        <div class="block">
            <div class="block-header"><h2>{title}</h2></div>
            <pre class="block-details">{details}</pre>
        </div>
    "#,
    title=status.canonical_reason().unwrap_or("Unknown Error"),
    details=error.to_string())
}
