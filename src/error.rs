use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
  MissingExtension,
  MissingPathComponent,
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::MissingExtension => "Missing request extension",
      Error::MissingPathComponent => "Missing path component",
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use std::error::Error;
    f.write_str(self.description())
  }
}
