use handler::base::*;

use std::io;

use iron::headers::{ CacheControl, CacheDirective, Vary, Pragma, Expires, HttpDate, ContentEncoding, Encoding };
use iron::mime::Mime;
use iron::modifiers::Header;
use iron::response::WriteBody;
use unicase::UniCase;
use time;
use flate2::FlateReadExt;

use git_ship::upload_pack;

#[derive(Clone)]
pub struct UploadPack;

fn body_thing<'a>(req: &'a mut Request) -> Result<Box<io::Read + 'a>, Error> {
    let encoding = if let Some(&ContentEncoding(ref encodings)) = req.headers.get() {
        if encodings.len() != 1 {
            return Err(Error::from("Can't handle multiple encodings"));
        }
        encodings[0].clone()
    } else {
        Encoding::Identity
    };
    Ok(match encoding {
        Encoding::Identity => Box::new(&mut req.body) as Box<io::Read>,
        Encoding::Gzip => Box::new((&mut req.body).gz_decode()?) as Box<io::Read>,
        Encoding::Deflate => Box::new((&mut req.body).deflate_decode()) as Box<io::Read>,
        encoding => return Err(Error::from(format!("Can't handle encoding {}", encoding))),
    })
}

impl Handler for UploadPack {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let no_cache = (
            Header(CacheControl(vec![
                CacheDirective::NoCache,
                CacheDirective::MaxAge(0),
                CacheDirective::MustRevalidate,
            ])),
            Header(Expires(HttpDate(time::empty_tm()))),
            Header(Pragma::NoCache),
            Header(Vary::Items(vec![
                UniCase("accept-encoding".to_owned()),
            ])),
        );
        let context = itry!(req.extensions.remove::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
        let mut body = itry!(body_thing(req), (status::BadRequest, no_cache));
        let response = itry!(upload_pack::prepare(context.repository, &mut *body), (status::BadRequest, no_cache));
        println!("upload_pack response: {:?}", response);
        let status_code = status::Unregistered(response.status_code());
        let mime: Mime = response.mime_type().parse().unwrap();
        Ok(Response::with((status_code, mime, no_cache, Box::new(W(response)) as Box<WriteBody>)))
    }
}

struct W(upload_pack::Response);
impl WriteBody for W {
    fn write_body(&mut self, res: &mut io::Write) -> io::Result<()> {
        self.0.write_to(res)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }
}

impl Route for UploadPack {
    fn method() -> Method {
        Method::Post
    }

    fn route() -> Cow<'static, str> {
        "/git-upload-pack".into()
    }
}
