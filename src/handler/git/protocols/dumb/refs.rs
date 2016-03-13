use handler::base::*;

use git2;

#[derive(Clone)]
pub struct Refs;

fn format_ref(reff: git2::Reference) -> Result<String, Error> {
  let target = try!(try!(reff.resolve()).target().ok_or(Error::from("Ref missing target")));
  let name = try!(reff.name().ok_or(Error::from("Ref missing name")));
  Ok(format!("{}\t{}", target, name))
}

fn format_refs(refs: git2::References) -> Result<String, Error> {
  let mut result = String::new();
  for reff in refs {
    result.push_str(&try!(format_ref(try!(reff))));
    result.push('\n');
  }
  Ok(result)
}

impl Handler for Refs {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let mime = mime!(Text/Plain; Charset=Utf8);
    let buffer = itry!(context.repository.references().map_err(From::from).and_then(format_refs), status::InternalServerError);
    Ok(Response::with((status::Ok, mime, buffer)))
  }
}

impl Route for Refs {
  fn method() -> Method {
    Method::Get
  }

  fn route() -> Cow<'static, str> {
    "/info/refs".into()
  }
}
