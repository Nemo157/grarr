use std::io;
use std::borrow::Borrow;

use super::url::Url;
use super::git2::{self, Oid};

use super::{pkt_line, Capability, Capabilities};

#[derive(Debug)]
pub struct UploadPack {
    head: Oid,
    refs: Vec<(String, Oid)>,
    capabilities: Capabilities,
}

#[derive(Debug)]
pub enum Response {
    UploadPack(UploadPack),
    Error(&'static str),
}

pub fn prepare(repo: &git2::Repository, url: &Url) -> Result<Response, git2::Error> {
    let service = url.query_pairs()
        .find(|&(ref key, _)| key == "service")
        .map(|(_, id)| id.clone());
    let service = service.as_ref().map(Borrow::borrow);
    match service {
        Some("git-upload-pack") => {
            let head = repo.head()?.target().expect("TODO: Better handling of non-HEAD containing repos");
            let refs = repo.references()?
                .map(|r| {
                    let r = r?;
                    let name = r.name()
                        .expect("TODO: Better handling of non-unicode refs")
                        .to_owned();
                    let target = r.resolve()?
                        .target()
                        .expect("Resolved references always have a target");
                    Ok((name, target))
                })
                .collect::<Result<Vec<_>, git2::Error>>()?;
            // TODO: Sort refs by name in C locale
            let capabilities = Capabilities::new(vec![
                Capability::SideBand,
                Capability::SideBand64K,
                Capability::MultiAck,
                Capability::MultiAckDetailed,
            ]);
            Ok(Response::UploadPack(UploadPack {
                head: head,
                refs: refs,
                capabilities: capabilities,
            }))
        }
        Some(_) => Ok(Response::Error("Unknown git service name")),
        None => Ok(Response::Error("Please upgrade your git client.")),
    }
}

impl UploadPack {
    pub fn write_to(&self, mut writer: &mut io::Write) -> io::Result<()> {
        pkt_line::write_str(&mut writer, "# service=git-upload-pack")?;
        pkt_line::flush(&mut writer)?;
        pkt_line::write_str(&mut writer, format!("{} HEAD\0{}", self.head, self.capabilities))?;
        for &(ref name, ref target) in &self.refs {
            pkt_line::write_str(&mut writer, format!("{} {}", target, name))?;
        }
        pkt_line::flush(&mut writer)?;
        Ok(())
    }
}

impl Response {
    pub fn status_code(&self) -> u16 {
        match *self {
            Response::UploadPack(_) => 200,
            Response::Error(_) => 403,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match *self {
            Response::UploadPack(_) => "application/x-git-upload-pack-advertisement",
            Response::Error(_) => "text/plain; charset=utf-8",
        }
    }

    pub fn write_to(&self, mut writer: &mut io::Write) -> io::Result<()> {
        match *self {
            Response::UploadPack(ref pack) => pack.write_to(writer),
            Response::Error(ref msg) => writer.write_all(msg.as_bytes()),
        }
    }
}
