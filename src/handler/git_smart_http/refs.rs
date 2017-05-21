use std::io;

use handler::base::*;
use git_ship::refs;
use iron::mime::Mime;
use iron::response::WriteBody;

#[derive(Clone)]
pub struct Refs;

impl Handler for Refs {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
        let response = itry!(refs::prepare(&context.repository, &req.url.clone().into()), status::InternalServerError);
        println!("response: {:?}", response);
        let status_code = status::Unregistered(response.status_code());
        let mime: Mime = response.mime_type().parse().unwrap();
        Ok(Response::with((status_code, mime, Box::new(W(response)) as Box<WriteBody>)))
    }
}

struct W(refs::Response);
impl WriteBody for W {
    fn write_body(&mut self, res: &mut io::Write) -> io::Result<()> {
        self.0.write_to(res)
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
