use std::io::{ self, Write };

pub trait WritePktLine {
  fn write_pkt_line<S: AsRef<str>>(&mut self, buf: S) -> io::Result<()>;
  fn write_pkt_line_binary<B: AsRef<[u8]>>(&mut self, buf: B) -> io::Result<()>;
  fn write_pkt_line_flush(&mut self) -> io::Result<()>;
}

impl WritePktLine for Vec<u8> {
  fn write_pkt_line<S: AsRef<str>>(&mut self, buf: S) -> io::Result<()> {
    let buf = buf.as_ref().as_bytes();
    if buf.len() >= 65520 {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "The maximum length of a pkt-line's data component is 65520 bytes (including LF)."));
    }
    if buf.is_empty() {
      return Ok(());
    }
    try!(write!(self, "{:04x}", buf.len() + 5));
    try!(self.write_all(buf));
    self.write_all(b"\n")
  }

  fn write_pkt_line_binary<B: AsRef<[u8]>>(&mut self, buf: B) -> io::Result<()> {
    let buf = buf.as_ref();
    if buf.len() > 65520 {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "The maximum length of a pkt-line's data component is 65520 bytes."));
    }
    if buf.is_empty() {
      return Ok(());
    }
    try!(write!(self, "{:04x}", buf.len() + 4));
    self.write_all(buf.as_ref())
  }

  fn write_pkt_line_flush(&mut self) -> io::Result<()> {
    self.write_all(b"0000")
  }
}
