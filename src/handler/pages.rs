use super::base::*;
use tree_entry;

use git2;

#[derive(Clone)]
pub struct Pages;

impl Handler for Pages {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    {
      let mut context = itry!(req.extensions.get_mut::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
      context.reference = Some("gh-pages".to_owned());
    }
    let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::MissingExtension), status::InternalServerError);
    let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);

    let mut path = router.find("path").unwrap_or("").to_owned();
    if path == "" || path.ends_with('/') {
      path = path + "index.html";
    }

    let entry = try!(tree_entry::get_tree_entry(&context, &path)).entry;

    match entry.kind() {
      Some(git2::ObjectType::Blob) => {
        let blob = entry.as_blob().unwrap();
        Ok(Response::with((status::Ok, utils::blob_mime(blob, &path), blob.content())))
      },
      _ => {
        Err(IronError::new(Error::from("Not found"), status::NotFound))
      },
    }
  }
}

impl Route for Pages {
  fn method() -> Method {
    Method::Get
  }

  fn routes() -> Vec<Cow<'static, str>> {
    vec![
      "/pages/".into(),
      "/pages/*path".into(),
    ]
  }
}
