use handler::base::*;
use git_ship::pkt_line;

use git2;

#[derive(Clone)]
pub struct Refs;

fn format_ref(reff: git2::Reference) -> Result<String, Error> {
    let target = try!(try!(reff.resolve()).target().ok_or(Error::from("Ref missing target")));
    let name = try!(reff.name().ok_or(Error::from("Ref missing name")));
    Ok(format!("{} {}", target, name))
}

fn format_refs(head: git2::Reference, refs: git2::References) -> Result<Vec<u8>, Error> {
    let mut result = Vec::new();
    pkt_line::write_str(&mut result, "# service=git-upload-pack")?;
    pkt_line::flush(&mut result)?;
    let head_id = head.target().ok_or(Error::from("HEAD missing target"))?;
    pkt_line::write_str(&mut result, format!("{} HEAD\0{}", head_id, "side-band side-band-64k multi_ack_detailed"))?;
    // TODO: Sort refs by name in C locale
    for reff in refs {
        pkt_line::write_str(&mut result, try!(format_ref(try!(reff))))?;
    }
    pkt_line::flush(&mut result)?;
    Ok(result)
}

#[allow(deprecated)]
fn find_service(req: &Request) -> Option<String> {
    req.url.clone().into_generic_url()
        .query_pairs()
        .into_iter()
        .find(|&(ref key, _)| key == "service")
        .map(|(_, ref id)| id.to_string())
}

impl Handler for Refs {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let context = itry!(req.extensions.get::<RepositoryContext>().ok_or(Error::from("missing extension")), status::InternalServerError);
        match find_service(req).as_ref().map(|s| &**s) {
            Some("git-upload-pack") => {
                let head = itry!(context.repository.head());
                let refs = itry!(context.repository.references());
                let buffer = itry!(format_refs(head, refs), status::InternalServerError);
                Ok(Response::with((
                    status::Ok,
                    mime!(Application/("x-git-upload-pack-advertisement")),
                    buffer)))
            },
            Some(_) => {
                Ok(Response::with((
                    status::Forbidden,
                    mime!(Text/Plain; Charset=Utf8),
                    "Unknown git service name")))
            }
            None => {
                // Assumed dumb client
                Ok(Response::with((
                    status::Forbidden,
                    mime!(Text/Plain; Charset=Utf8),
                    "Please upgrade your git client.")))
            }
        }
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
