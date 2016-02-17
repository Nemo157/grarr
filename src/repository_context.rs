use git2;
use std::fs;
use std::path::{ Path, PathBuf };
use std::borrow::Cow;
use typemap::Key;
use router::Router;
use iron::IronResult;
use iron::request::Request;
use iron::response::Response;
use iron::middleware::{ Handler };
use iron::status;
use hyper::method::Method;
use handler::route::Route;
use error::Error;

pub struct RepositoryContext {
  pub requested_path: PathBuf,
  pub canonical_path: PathBuf,
  pub repository: git2::Repository,
  reference: Option<String>,
}

impl Key for RepositoryContext {
  type Value = RepositoryContext;
}

impl RepositoryContext {
  pub fn reference(&self) -> Result<git2::Reference, Error> {
    self.reference.as_ref()
      .ok_or("No reference specified".into())
      .and_then(|r| self.repository.revparse_ext(r).map_err(From::from))
      .and_then(|(_, r)| r.ok_or("Commit ref did not produce an intermediate reference".into()))
  }

  pub fn commit(&self) -> Result<git2::Commit, Error> {
    self.reference.as_ref()
      .ok_or("No commit ref specified".into())
      .and_then(|r| self.repository.revparse_single(r).map_err(From::from))
      .map(|obj| obj.id())
      .and_then(|id| self.repository.find_commit(id).map_err(From::from))
  }
}

pub fn inject_repository_context<H: Handler>(root: &Path, handler: H) -> RepositoryContextHandler<H> {
  RepositoryContextHandler {
    canonical_root: fs::canonicalize(root).unwrap(),
    handler: handler,
  }
}

#[derive(Clone)]
pub struct RepositoryContextHandler<H: Handler> {
  canonical_root: PathBuf,
  handler: H,
}

impl<H: Handler> Handler for RepositoryContextHandler<H> {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let (requested_path, reference) = {
      let router = itry!(req.extensions.get::<Router>().ok_or(Error::MissingExtension), status::InternalServerError);
      (router.find("repo").map(ToOwned::to_owned), router.find("ref").map(ToOwned::to_owned))
    };
    let requested_path = PathBuf::from(itry!(requested_path.ok_or(Error::MissingPathComponent), status::InternalServerError));
    let full_path = self.canonical_root.join(&requested_path);
    let full_canonical_path = itry!(fs::canonicalize(&full_path), status::NotFound);
    let canonical_path = itry!(full_canonical_path.strip_prefix(&self.canonical_root), status::InternalServerError).to_owned();
    let repository = itry!(git2::Repository::open(self.canonical_root.join(&requested_path)), status::NotFound);
    req.extensions.insert::<RepositoryContext>(RepositoryContext {
      requested_path: requested_path,
      canonical_path: canonical_path,
      repository: repository,
      reference: reference,
    });
    self.handler.handle(req)
  }
}

impl<'a, H: Handler + Route> Route for RepositoryContextHandler<H> {
  fn method() -> Method { H::method() }
  fn routes() -> Vec<Cow<'static, str>> { H::routes().into_iter().map(|r| ("/*repo".to_owned() + &r).into()).collect() }
}
