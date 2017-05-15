use super::base::*;
use settings;
// use params::{ Map, Params, Value };

#[derive(Clone)]
pub struct Settings;
#[derive(Clone)]
pub struct SettingsPost;

impl Handler for Settings {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let settings = itry!(req.extensions.get::<settings::Settings>().ok_or(Error::from("missing extension")), status::InternalServerError);
    Html {
      render: render::Settings(settings),
      etag: None,
      req: req,
    }.into()
  }
}

impl Route for Settings {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/-/settings".into()
  }
}

impl Handler for SettingsPost {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
  /*
    let settings = {
      let map: Map = itry!(req.get::<Params>(), status::InternalServerError);
      let settings = itry!(req.extensions.get::<settings::Settings>().ok_or(Error::from("missing extension")), status::InternalServerError);
      settings.with(
        map.iter().filter_map(|(key, value)| match *value {
          Value::String(ref value) => Some((&**key, &**value)),
          _ => None,
        }))
    };
    println!("{:?}", settings);
    let html = Html {
      render: &render::Settings(&settings),
      etag: None,
      req: req,
    };
    Ok(Response::with((status::SeeOther, Redirect(req.url.clone()), html, &settings)))
  */
    Ok(Response::with((status::SeeOther, Redirect(req.url.clone()))))
  }
}

impl Route for SettingsPost {
  fn method() -> Method {
    Method::Post
  }

  fn route() -> Cow<'static, str> {
    "/-/settings".into()
  }
}
