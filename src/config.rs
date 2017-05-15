use std::{ env, ffi, io };
use std::fs::File;
use std::io::Read;
use std::path::{ Path, PathBuf };
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub repos: Repos,
  pub avatars: Avatars,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repos {
  pub root: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avatars {
  pub cache: Cache,
  pub gravatar: Gravatar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
  pub enable: bool,
  pub capacity: usize,
  pub ttl_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gravatar {
  pub enable: bool,
}

error_chain! {
    foreign_links {
        Io(io::Error);
        Toml(toml::de::Error);
    }
}

pub fn load<S: AsRef<ffi::OsStr> + ?Sized>(from: Option<&S>) -> Result<Config> {
  let path = if let Some(path) = from.map(Path::new) {
    path.canonicalize()?
  } else {
    env::current_dir()?.canonicalize()?
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
    let mut text = String::new();
    File::open(path)?.read_to_string(&mut text)?;
    let mut config: Config = toml::from_str(text.as_str())?;
    config.repos.root = config.repos.root.canonicalize()?;
    Ok(config)
  }
}
