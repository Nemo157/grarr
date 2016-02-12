use std::fmt;
use std::borrow::Cow;
use std::path::Path;

use mime::Mime;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use iron::headers::{ EntityTag };

#[macro_export]
macro_rules! file {
  ($x:expr) => ({
    let bytes = include_bytes!($x);
    $crate::handler::utils::File(
      $crate::handler::utils::mime($x),
      ::iron::headers::EntityTag::strong(
        $crate::handler::utils::sha1(bytes)),
      ::std::borrow::Cow::Borrowed(bytes))
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

pub fn sha1(file: &[u8]) -> String {
  let mut hasher = Sha1::new();
  hasher.input(file);
  hasher.result_str()
}

