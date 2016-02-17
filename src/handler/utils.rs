use std::fmt;
use std::borrow::Cow;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use mime::Mime;
use iron::headers::{ EntityTag };

#[macro_export]
macro_rules! file {
  ($x:expr) => ({
    let bytes = include_bytes!($x);
    $crate::handler::utils::File(
      $crate::handler::utils::mime($x),
      ::iron::headers::EntityTag::strong(sha1!(bytes as &[u8])),
      ::std::borrow::Cow::Borrowed(bytes))
  });
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
    sha1!(env!("CARGO_PKG_VERSION"))
  });
  ($($x:expr),+) => ({
    sha1!(env!("CARGO_PKG_VERSION"), $($x),*)
  });
}

#[derive(Clone)]
pub struct File(pub Mime, pub EntityTag, pub Cow<'static, [u8]>);

impl fmt::Debug for File {
  fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
    write!(w, "File{:?}", (&self.0, &self.1, &self.2.len()))
  }
}


pub fn mime(path: &str) -> Mime {
  match Path::new(path).extension().and_then(|s| s.to_str()) {
    Some("css") => mime!(Text/Css),
    Some("js") => mime!(Text/Javascript),
    None | Some(_) => mime!(Application/("octet-stream")),
  }
}

pub fn sha1<T: AsRef<[u8]>>(file: T) -> String {
  let mut hasher = Sha1::new();
  hasher.input(file.as_ref());
  hasher.result_str()
}
