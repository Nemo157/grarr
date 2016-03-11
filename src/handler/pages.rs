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
    let path = router.find("path").unwrap_or("");
    let tree_entry::TreeEntryContext {
      entry,
      entry_path: mut path,
      ..
    } = try!(tree_entry::get_tree_entry(&context, path));

    let mut new_entry = None;
    if path.ends_with('/') {
      if let Some(git2::ObjectType::Tree) = entry.kind() {
        if let Some(tree_entry) = entry.as_tree().unwrap().get_name("index.html") {
          new_entry = Some(itry!(tree_entry.to_object(&context.repository), status::InternalServerError));
          path = path + "index.html";
        }
      }
    }
    let entry = new_entry.unwrap_or(entry);

    match entry.kind() {
      Some(git2::ObjectType::Blob) => {
        Ok(Response::with((status::Ok, utils::mime(&*path), entry.as_blob().unwrap().content())))
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
