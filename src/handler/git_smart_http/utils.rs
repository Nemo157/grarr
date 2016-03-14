use std::io::{ self, Write };
use std::str;

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

pub struct PktLines<'a> {
  source: &'a mut io::Read,
}

pub trait ReadPktLine {
  fn read_pkt_line(&mut self, buf: &mut [u8]) -> io::Result<usize>;
  fn pkt_lines(&mut self) -> PktLines;
}

impl<T: io::Read> ReadPktLine for T {
  fn read_pkt_line(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let mut size_buf = [0; 4];
    try!(self.read_exact(&mut size_buf));
    let size_str = try!(str::from_utf8(&size_buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
    let size = try!(usize::from_str_radix(size_str, 16).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
    if size == 0 {
      return Ok(0);
    }
    if size < 4 {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "Size less than 4 and not equal to 0"));
    }
    let size = size - 4;
    if size > buf.len() {
      return Err(io::Error::new(io::ErrorKind::InvalidInput, "Buffer was not large enough"));
    }
    try!(self.read_exact(&mut buf[..size]));
    if size > 0 && buf[size - 1] == b'\n' {
      Ok(size - 1)
    } else {
      Ok(size)
    }
  }

  fn pkt_lines(&mut self) -> PktLines {
    PktLines { source: self }
  }
}

impl<'a> Iterator for PktLines<'a> {
  type Item = Result<String, io::Error>;
  fn next(&mut self) -> Option<Result<String, io::Error>> {
    let mut buf = vec![0; 65520];
    match self.source.read_pkt_line(&mut buf) {
      Ok(len) => {
        buf.truncate(len);
        Some(String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
      },
      Err(e) => match e.kind() {
        io::ErrorKind::UnexpectedEof => None,
        _ => Some(Err(e)),
      },
    }
  }
}
