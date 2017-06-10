use std::fmt;
use std::borrow::Cow;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use mime::Mime;
use iron::headers::EntityTag;
use git2;

use assets::File;

#[macro_export]
macro_rules! file {
    ($x:expr) => ((
        $crate::handler::utils::FileData::from((
            $crate::handler::utils::mime($x),
            include_bytes!($x).as_ref()))
    ));
}

#[macro_export]
macro_rules! sha1 {
    ($($x:expr),*) => ({
        use ::crypto::digest::Digest;
        let mut hasher = ::crypto::sha1::Sha1::new();
        $(hasher.input(::std::convert::AsRef::<[u8]>::as_ref($x));)*
        hasher.result_str()
    });
}

#[macro_export]
macro_rules! versioned_sha1 {
    () => ({
        sha1!(env!("CARGO_PKG_VERSION"), ::REVISION.unwrap_or_default())
    });
    ($($x:expr),+) => ({
        sha1!(env!("CARGO_PKG_VERSION"), ::REVISION.unwrap_or_default(), $($x),*)
    });
}

#[derive(Clone)]
pub struct FileData(pub Mime, pub EntityTag, pub Cow<'static, [u8]>);

impl fmt::Debug for FileData {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "FileData{:?}", (&self.0, &self.1, &self.2.len()))
    }
}

impl From<&'static File> for FileData {
    fn from(file: &'static File) -> FileData {
        FileData::from((mime(file.name()), file.contents))
    }
}

impl<B> From<(Mime, B)> for FileData where B: Into<Cow<'static, [u8]>> {
    fn from((mime, content): (Mime, B)) -> FileData {
        let content = content.into();
        let etag = EntityTag::strong(sha1(content.as_ref()));
        FileData(mime, etag, content)
    }
}

pub fn mime(path: &str) -> Mime {
    match Path::new(path).extension().and_then(|s| s.to_str()) {
        Some("css") => mime!(Text/Css),
        Some("html") => mime!(Text/Html),
        Some("js") => mime!(Text/Javascript),
        None | Some(_) => mime!(Application/("octet-stream")),
    }
}

pub fn blob_mime(blob: &git2::Blob, path: &str) -> Mime {
    match Path::new(path).extension().and_then(|s| s.to_str()) {
        Some("css") => mime!(Text/Css),
        Some("html") => mime!(Text/Html),
        Some("js") => mime!(Text/Javascript),
        None | Some(_) => {
            if blob.is_binary() {
                mime!(Application/("octet-stream"))
            } else {
                mime!(Text/Plain)
            }
        },
    }
}

pub fn sha1<T: AsRef<[u8]>>(file: T) -> String {
    let mut hasher = Sha1::new();
    hasher.input(file.as_ref());
    hasher.result_str()
}

pub trait CacheMatches {
    fn cache_matches(&self, etag: &EntityTag) -> bool;
}

#[cfg(not(all(feature = "maybe_cache", feature = "cache")))]
mod caching {
    use std::time::Duration;
    use iron::headers::EntityTag;
    use iron::headers::Vary;
    use iron::modifiers::Header;
    use iron::request::Request;
    use unicase::UniCase;
    use super::CacheMatches;

    // In debug mode assume the etag never matches so we
    // don't have to bump version numbers for dynamic content
    impl<'a, 'b> CacheMatches for Request<'a, 'b> {
        fn cache_matches(&self, _etag: &EntityTag) -> bool {
            false
        }
    }

    // Should return () once https://github.com/reem/rust-modifier/pull/19 is merged
    pub fn cache_headers_for(_entity_tag: &EntityTag, _duration: Duration) -> Header<Vary> {
        Header(Vary::Items(vec![
            UniCase("accept-encoding".to_owned()),
        ]))
    }
}

#[cfg(all(feature = "maybe_cache", feature = "cache"))]
mod caching {
    use std::time::Duration;
    use iron::headers::EntityTag;
    use iron::headers::{ ETag, CacheControl, CacheDirective, Vary };
    use iron::modifiers::Header;
    use iron::request::Request;
    use unicase::UniCase;
    use super::CacheMatches;

    impl<'a, 'b> CacheMatches for Request<'a, 'b> {
        fn cache_matches(&self, etag: &EntityTag) -> bool {
            use iron::headers::IfNoneMatch;
            if let Some(&IfNoneMatch::Items(ref items)) = self.headers.get() {
                if items.len() == 1 && items[0] == *etag {
                    return true;
                }
            }
            false
        }
    }

    // Where's my abstract return types....
    pub fn cache_headers_for(entity_tag: &EntityTag, duration: Duration)
        -> (Header<CacheControl>, Header<ETag>, Header<Vary>)
    {
        (
            Header(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(duration.as_secs() as u32),
            ])),
            Header(ETag(entity_tag.clone())),
            Header(Vary::Items(vec![
                UniCase("accept-encoding".to_owned()),
            ])),
        )
    }
}

pub use self::caching::*;
