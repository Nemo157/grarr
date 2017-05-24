use std::fmt;
use std::rc::Rc;
use std::sync::Mutex;
use std::io;
use std::borrow::Cow;

use super::git2::{self, Oid, PackBuilderStage};

use super::{pkt_line, PktLine, Capability, Capabilities, Multiplexer, Result};

pub struct Pack {
    repo: git2::Repository,
    commits: Vec<Oid>,
    common: Vec<Oid>,
    capabilities: Capabilities,
}

#[derive(Debug)]
pub struct Continue {
    common: Vec<Oid>,
    capabilities: Capabilities,
}

#[derive(Debug)]
pub struct Request {
    wants: Vec<Oid>,
    haves: Vec<Oid>,
    capabilities: Capabilities,
    done: bool,
}

#[derive(Debug)]
pub enum Response {
    Pack(Pack),
    Continue(Continue),
    Error(Cow<'static, str>),
}

fn parse(body: &mut io::Read) -> Result<Request> {
    let mut request = Request {
        wants: Vec::new(),
        haves: Vec::new(),
        capabilities: Capabilities::empty(),
        done: false,
    };

    pkt_line::each_str(body, |line| {
        println!("line {:?}", line);
        let line = match line {
            PktLine::Flush => return Ok(()),
            PktLine::Line(line) => line,
        };
        if line.len() < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unexpected pkt-line {}", line)));
        }
        match &line[0..4] {
            "want" => {
                let line = line[5..].trim();
                let (want, caps) = line.split_at(line.find(' ').unwrap_or(line.len()));
                request.wants.push(want.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?);
                if !caps.is_empty() {
                    request.capabilities = caps.parse().unwrap();
                }
            },
            "have" => {
                request.haves.push(line[5..].trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?);
            },
            "done" => {
                request.done = true;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unexpected pkt-line {}", line))),
        }
        Ok(())
    })?;

    println!("request: {:?}", request);
    Ok(request)
}

fn refs(repo: &git2::Repository) -> Result<Vec<Oid>> {
    repo.references()?
        .map(|r| {
            let r = r?;
            Ok(r.resolve()?
                .target()
                .expect("Resolved references always have a target"))
        })
        .collect()
}

fn graph_ancestor_of_any(repo: &git2::Repository, commit: Oid, descendants: &[Oid]) -> Result<bool> {
    for &descendant in descendants {
        if repo.graph_descendant_of(descendant, commit)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn graph_descendant_of_any(repo: &git2::Repository, commit: Oid, ancestors: &[Oid]) -> Result<bool> {
    for &ancestor in ancestors {
        if repo.graph_descendant_of(commit, ancestor)? {
            return Ok(true);
        }
    }
    Ok(false)
}

// a commit set is closed if every commit in `descendants` has at least one ancestor in `ancestors`
fn is_closed(repo: &git2::Repository, descendants: &[Oid], ancestors: &[Oid]) -> Result<bool> {
    for &descendant in descendants {
        if !graph_descendant_of_any(repo, descendant, ancestors)? {
            println!("{:?} has no ancestors in {:?}", descendant, ancestors);
            return Ok(false);
        }
    }
    Ok(true)
}

#[allow(collapsible_if)]
fn compute_response(repo: git2::Repository, refs: Vec<Oid>, request: Request) -> Result<Response> {
    let mut common = Vec::<Oid>::new();
    // for each id given in have
    for id in request.haves {
        // if it is an ancestor of a ref
        if graph_ancestor_of_any(&repo, id, &refs)? {
            // and is not an ancestor of a common
            if !graph_ancestor_of_any(&repo, id, &common)? {
                // add it to common
                common.push(id);
            }
        }
    }

    println!("common: {:?}", common);

    if request.done || is_closed(&repo, &request.wants, &common)? {
        let commits = {
            let mut revwalk = repo.revwalk()?;
            for id in request.wants {
                revwalk.push(id)?;
            }
            for &id in &common {
                revwalk.hide(id)?;
            }
            revwalk.collect::<::std::result::Result<_, git2::Error>>()?
        };
        Ok(Response::Pack(Pack {
            repo: repo,
            commits: commits,
            common: common,
            capabilities: request.capabilities
        }))
    } else {
        Ok(Response::Continue(Continue {
            common: common,
            capabilities: request.capabilities
        }))
    }
}

pub fn prepare(repo: git2::Repository, body: &mut io::Read) -> Result<Response> {
    let request = parse(body)?;
    let refs = refs(&repo)?;
    if request.wants.is_empty() {
        return Ok(Response::Error("need wants".into()));
    }
    for id in &request.wants {
        if !refs.contains(&id) {
            return Ok(Response::Error(format!("want missing from refs {}", id).into()));
        }
    }
    Ok(compute_response(repo, refs, request)?)
}

impl Pack {
    pub fn write_to(&mut self, mut writer: &mut io::Write) -> Result<()> {
        if !self.common.is_empty() && self.capabilities.contains(Capability::MultiAckDetailed) {
            for id in &self.common {
                let line = format!("ACK {} common", id);
                println!("{}", line);
                pkt_line::write_str(&mut writer, line)?;
            }
            let line = format!("ACK {}", self.common.iter().last().unwrap());
            println!("{}", line);
            pkt_line::write_str(&mut writer, line)?;
        } else {
            pkt_line::write_str(&mut writer, "NAK")?;
        }
        let mut output = Multiplexer::new(writer, &self.capabilities)?;
        {
            let output = Rc::new(Mutex::new(&mut output));
            let mut builder = self.repo.packbuilder()?;
            {
                let output = output.clone();
                let mut first_delta = true;
                builder.set_progress_callback(move |stage, current, total| {
                    let mut output = output.lock().unwrap();
                    match stage {
                        PackBuilderStage::AddingObjects => {
                            let _ = output.write_progress(format!("Counting objects {}\r", current));
                        }
                        PackBuilderStage::Deltafication => {
                            if first_delta {
                                let _ = output.write_progress("\n");
                                first_delta = false;
                            }
                            let percent = (current as f64 / total as f64) * 100.0;
                            if current == total {
                                let _ = output.write_progress(format!(
                                    "Compressing objects: {:.0}% ({}/{}), done\n",
                                    percent, current, total));
                            } else {
                                let _ = output.write_progress(format!(
                                    "Compressing objects: {:.0}% ({}/{})\r",
                                    percent, current, total));
                            }
                        }
                    }
                    true
                })?;
            }
            for &id in &self.commits {
                builder.insert_commit(id)?;
            }
            builder.foreach(|object| output.lock().unwrap().write_packfile(object).is_ok())?;
        }
        pkt_line::flush(output.into_inner())?;
        Ok(())
    }
}

impl Continue {
    pub fn write_to(&self, mut writer: &mut io::Write) -> Result<()> {
        if self.capabilities.contains(Capability::MultiAckDetailed) {
            for id in &self.common {
                let line = format!("ACK {} common", id);
                println!("{}", line);
                pkt_line::write_str(&mut writer, line)?;
            }
        } else {
            // TODO
        }
        pkt_line::write_str(&mut writer, "NAK")?;
        Ok(())
    }
}

impl Response {
    pub fn status_code(&self) -> u16 {
        match *self {
            Response::Pack(_) => 200,
            Response::Continue(_) => 200,
            Response::Error(_) => 403,
        }
    }

    pub fn write_to(&mut self, mut writer: &mut io::Write) -> Result<()> {
        match *self {
            Response::Pack(ref mut pack) => pack.write_to(writer)?,
            Response::Continue(ref c) => c.write_to(writer)?,
            Response::Error(ref msg) => writer.write_all(msg.as_bytes())?,
        }
        Ok(())
    }
}

impl fmt::Debug for Pack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct R<'a>(&'a git2::Repository);
        impl<'a> fmt::Debug for R<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("git2::Repository")
                    .field("path", &self.0.path())
                    .finish()
            }
        }
        f.debug_struct("Pack")
            .field("repo", &R(&self.repo))
            .field("commits", &self.commits)
            .field("common", &self.common)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}
