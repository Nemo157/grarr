use std::error;
use std::fmt;
use std::io;
use git2;

#[derive(Debug)]
pub struct Error(Box<error::Error + Send + Sync>);

impl error::Error for Error {
  fn description(&self) -> &str {
    self.0.description()
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl From<git2::Error> for Error {
  fn from(e: git2::Error) -> Error {
    Error(e.into())
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Error {
    Error(e.into())
  }
}

impl From<&'static str> for Error {
  fn from(s: &'static str) -> Error {
    Error(s.into())
  }
}

impl From<String> for Error {
  fn from(s: String) -> Error {
    Error(s.into())
  }
}
