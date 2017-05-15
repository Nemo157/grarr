use super::base::*;
use tree_entry;

use std::str;
use pulldown_cmark;
use git2;

#[derive(Clone)]
pub struct Pages;

fn get_response(context: &RepositoryContext, path: &str) -> Result<Response, ()> {
    let entry = try!(tree_entry::get_tree_entry(&context, &path).map_err(|_| ())).entry;

    match entry.kind() {
        Some(git2::ObjectType::Blob) => {
            let blob = entry.as_blob().unwrap();
            Ok(Response::with((status::Ok, utils::blob_mime(blob, &path), blob.content())))
        },
        _ => Err(()),
    }
}

fn get_markdown_response(context: &RepositoryContext, path: &str) -> Result<Response, ()> {
    if !path.ends_with(".html") {
        return Err(());
    }

    let md_path = path[..path.len()-5].to_owned() + ".md";
    let entry = try!(tree_entry::get_tree_entry(&context, &md_path).map_err(|_| ())).entry;

    match entry.kind() {
        Some(git2::ObjectType::Blob) => {
            let blob = entry.as_blob().unwrap();
            let content = try!(str::from_utf8(blob.content()).map_err(|_| ()));
            let mut buffer = String::with_capacity(content.len() * 3 / 2);
            let parser = pulldown_cmark::Parser::new(&content);
            pulldown_cmark::html::push_html(&mut buffer, parser);
            Ok(Response::with((status::Ok, mime!(Text/Html), buffer)))
        },
        _ => Err(()),
    }
}

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

        get_response(context, &*path)
            .or_else(|_| get_markdown_response(context, &*path))
            .or_else(|_| Err(IronError::new(Error::from("Not found"), status::NotFound)))
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
