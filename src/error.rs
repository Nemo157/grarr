use std::error;
use std::fmt;
use std::borrow::{ Cow, Borrow };
use git2;

#[derive(Debug)]
pub enum Error {
  MissingExtension,
  MissingPathComponent,
  String(Cow<'static, str>),
  Git(git2::Error),
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::MissingExtension => "Missing request extension",
      Error::MissingPathComponent => "Missing path component",
      Error::String(ref s) => s.borrow(),
      Error::Git(ref e) => e.description(),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Git(ref e) => e.fmt(f),
      _ => f.write_str(error::Error::description(self)),
    }
  }
}

impl From<git2::Error> for Error {
  fn from(e: git2::Error) -> Error {
    Error::Git(e)
  }
}

impl From<&'static str> for Error {
  fn from(s: &'static str) -> Error {
    Error::String(s.into())
  }
}

impl From<String> for Error {
  fn from(s: String) -> Error {
    Error::String(s.into())
  }
}
