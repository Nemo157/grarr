use std::{ env, ffi, io, fmt };
use std::borrow::ToOwned;
use std::fs::File;
use std::io::Read;
use std::path::{ Path, PathBuf };
use toml;
use rustc_serialize::{ Decoder, Decodable, Encoder, Encodable };

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
  pub repos: Repos,
  pub avatars: Avatars,
}

#[derive(Debug)]
// Cannot derive decode/encode as PathBuf by default uses a [u8/u16] for storage...
pub struct Repos {
  pub root: PathBuf,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Avatars {
  pub cache: Cache,
  pub gravatar: Gravatar,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Cache {
  pub enable: bool,
  pub capacity: usize,
  pub ttl_seconds: u64,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Gravatar {
  pub enable: bool,
}

#[derive(Debug)]
pub enum Error {
  IO(io::Error),
  Parse(Vec<toml::ParserError>),
  Decode(toml::DecodeError),
  Str(String),
}

pub fn load<S: AsRef<ffi::OsStr> + ?Sized>(from: Option<&S>) -> Result<Config, Error> {
  let path = if let Some(path) = from.map(Path::new) {
    try!(path.canonicalize())
  } else {
    try!(try!(env::current_dir()).canonicalize())
  };

  if path.is_dir() {
    Ok(Config {
      repos: Repos {
        root: path,
      },
      avatars: Avatars {
        cache: Cache {
          enable: true,
          capacity: 100,
          ttl_seconds: 60,
        },
        gravatar: Gravatar {
          enable: true,
        },
      },
    })
  } else {
    load_file(&path).and_then(|mut config| { config.repos.root = try!(config.repos.root.canonicalize()); Ok(config) })
  }
}

fn load_file(path: &Path) -> Result<Config, Error> {
  let text = try!(read_file(path));
  let value = try!(text.parse::<toml::Value>());
  Config::decode(&mut toml::Decoder::new(value)).map_err(From::from)
}

fn read_file(path: &Path) -> io::Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open(path));
  try!(file.read_to_string(&mut text));
  Ok(text)
}

impl<'a> From<&'a str> for Error {
  fn from(s: &'a str) -> Error {
    Error::Str(s.to_owned())
  }
}

impl From<Vec<toml::ParserError>> for Error {
  fn from(v: Vec<toml::ParserError>) -> Error {
    Error::Parse(v)
  }
}

impl From<toml::DecodeError> for Error {
  fn from(error: toml::DecodeError) -> Error {
    Error::Decode(error)
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Error {
    Error::IO(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::IO(ref error) => write!(w, "IO Error:\n{}", error),
      Error::Parse(ref errors) => {
        try!(write!(w, "Parse Error:\n"));
        for error in errors {
          try!(write!(w, "{}", error));
        }
        Ok(())
      },
      Error::Decode(ref error) => write!(w, "Decode Error:\n{}", error),
      Error::Str(ref s) => write!(w, "Misc Error:\n{}", s),
    }
  }
}

impl Decodable for Repos {
  fn decode<D: Decoder>(d: &mut D) -> Result<Repos, D::Error> {
    d.read_struct("repos", 1, |d| {
      Ok(Repos {
        root: try!(d.read_struct_field("root", 0, |d| d.read_str().map(From::from))),
      })
    })
  }
}

impl Encodable for Repos {
  fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
    e.emit_struct("repos", 1, |e| {
      e.emit_struct_field("root", 0, |e| e.emit_str(&self.root.to_string_lossy()))
    })
  }
}
