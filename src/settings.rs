use std::fmt;
use std::str::FromStr;
use iron::IronResult;
use iron::headers::{ Cookie, SetCookie };
use cookie::Cookie as CookiePair;
use iron::middleware::BeforeMiddleware;
use iron::request::Request;
use iron::response::Response;
use iron::modifiers::Header;
use iron::modifier::Modifier;
use typemap::Key;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(enum_variant_names)] // Co-inkidink
pub enum Theme {
  SolarizedDark,
  SolarizedLight,
}

impl fmt::Display for Theme {
  fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
    w.write_str(match *self {
      Theme::SolarizedDark => "solarized-dark",
      Theme::SolarizedLight => "solarized-light",
    })
  }
}

impl FromStr for Theme {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "solarized-dark" => Ok(Theme::SolarizedDark),
      "solarized-light" => Ok(Theme::SolarizedLight),
      _ => Err(format!("Unrecognized theme '{}'", s)),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Settings {
  pub theme: Theme,
}

impl Default for Settings {
  fn default() -> Settings {
    Settings {
      theme: Theme::SolarizedDark,
    }
  }
}

impl Key for Settings {
  type Value = Settings;
}

impl Settings {
  #[cfg_attr(feature = "clippy", allow(single_match))] // Will have more in the future
  pub fn with<'a, 'b, I: Iterator<Item=(&'a str, &'b str)>>(&self, settings: I) -> Settings {
    let mut result = self.clone();
    for (key, value) in settings {
      match key {
        "theme" => {
          if let Ok(theme) = value.parse().map_err(|e| println!("{}", e)) {
            result.theme = theme;
          }
        },
        _ => (),
      }
    }
    result
  }
}

impl<'a> Modifier<Response> for &'a Settings {
  fn modify(self, response: &mut Response) {
    let theme = CookiePair::build("theme", self.theme.to_string()).path("/").finish();
    Header(SetCookie(vec![theme.to_string()])).modify(response);
  }
}

impl BeforeMiddleware for Settings {
  fn before(&self, req: &mut Request) -> IronResult<()> {
    let settings = match req.headers.get() {
      Some(&Cookie(ref cookies)) =>
        self.with(cookies.iter().map(|c| {
            let pair = CookiePair::parse(c.as_ref()).unwrap();
            (pair.name_raw().unwrap(), pair.value_raw().unwrap())
        })),
      None =>
        self.clone(),
    };
    req.extensions.insert::<Settings>(settings);
    Ok(())
  }
}
