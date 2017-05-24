use std::{io, fmt, error, result};

use git2;

#[derive(Debug)]
pub enum Error {
    Git2(git2::Error),
    Io(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn cause(&self) -> &error::Error {
        match *self {
            Error::Git2(ref err) => err,
            Error::Io(ref err) => err,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.cause().description()
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self.cause())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "git_ship error: {}", self.cause())
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error { Error::Git2(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}
