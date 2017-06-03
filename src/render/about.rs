use { REVISION, DATE };
use std::fmt;
use super::utils::markdown;

pub fn about(_: ()) -> impl fmt::Display {
    let url: &'static str = "https://git.nemo157.com/grarr";

    fmt!(r#"
        <div class="block">
            <div class="block-header"><h3>About</h3></div>
            <div class="block-details">{readme}</div>
        </div>
        <div class="block">
            <div class="block-header"><h3>Version</h3></div>
            <div class="block-details">
                Website generated using
                <a href="{url}">grarr</a>
                version {version} {link}
            </div>
        </div>
    "#,
    readme=markdown(include_str!("../../README.md")),
    url=url,
    version=env!("CARGO_PKG_VERSION"),
    link={
        match (REVISION, DATE) {
            (Some(rev), None)
                => format!(r#"(<a href="{url}/commits/{rev}">{rev}</a>)"#, url=url, rev=rev),
            (None, Some(date))
                => format!("({})", date),
            (Some(rev), Some(date))
                => format!(r#"(<a href="{url}/commits/{rev}">{rev} {date}</a>)"#, url=url, rev=rev, date=date),
            (None, None)
                => String::new(),
        }
    })
}
