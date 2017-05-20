use std::io;
use std::cmp;

use super::pkt_line;
use super::{Capability, Capabilities};

/// A pkt-line multiplexer for the `side-band` and `side-band-64k` capabilities.
/// Also supports being a transparent passthrough when neither capability is
/// specified to make usage easier.
/// https://github.com/git/git/blob/10c78a162fa821ee85203165b805ff46be454091/Documentation/technical/protocol-capabilities.txt#L119
pub struct Multiplexer<'a> {
    writer: &'a mut io::Write,
    limit: Option<usize>,
    buffer: [u8; pkt_line::MAX_PKT_LINE_DATA_LEN],
}

impl<'a> Multiplexer<'a> {
    pub fn new(writer: &'a mut io::Write, caps: &Capabilities) -> io::Result<Multiplexer<'a>> {
        let limit = match (caps.contains(Capability::SideBand), caps.contains(Capability::SideBand64K)) {
            (true, true) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "client cannot send both side-band and side-band-64k")),
            (true, false) => Some(1000),
            (false, true) => Some(pkt_line::MAX_PKT_LINE_DATA_LEN),
            (false, false) => None,
        };
        Ok(Multiplexer { writer: writer, limit: limit, buffer: [0; pkt_line::MAX_PKT_LINE_DATA_LEN] })
    }

    pub fn write_packfile<B: AsRef<[u8]>>(&mut self, bytes: B) -> io::Result<()> {
        let bytes = bytes.as_ref();
        if let Some(limit) = self.limit {
            let mut offset = 0;
            while offset < bytes.len() {
                let len = cmp::min(bytes.len() - offset, limit - 1);
                self.buffer[0] = 1;
                self.buffer[1..len + 1].copy_from_slice(&bytes[offset..offset + len]);
                pkt_line::write(&mut self.writer, &self.buffer[0..len + 1])?;
                offset += len;
            }
            Ok(())
        } else {
            self.writer.write_all(bytes)
        }
    }

    pub fn write_progress<S: AsRef<str>>(&mut self, msg: S) -> io::Result<()> {
        let msg = msg.as_ref();
        if let Some(limit) = self.limit {
            if msg.len() > limit {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "msg too long"));
            }
            self.buffer[0] = 2;
            self.buffer[1..msg.len() + 1].copy_from_slice(msg.as_bytes());
            pkt_line::write(&mut self.writer, &self.buffer[0..msg.len() + 1])?;
            self.writer.flush()?;
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn write_error<S: AsRef<str>>(&mut self, msg: S) -> io::Result<()> {
        let msg = msg.as_ref();
        if let Some(limit) = self.limit {
            if msg.len() > limit {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "msg too long"));
            }
            self.buffer[0] = 3;
            self.buffer[1..msg.len() + 1].copy_from_slice(msg.as_bytes());
            pkt_line::write(&mut self.writer, &self.buffer[0..msg.len() + 1])?;
            self.writer.flush()?;
        }
        Ok(())
    }

    pub fn into_inner(self) -> &'a mut io::Write {
        self.writer
    }
}
