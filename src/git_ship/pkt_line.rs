use std::io;
use std::str;

const MAX_PKT_LINE_LEN: usize = 65520;
const PKT_LINE_SIZE_LEN: usize = 4;
pub const MAX_PKT_LINE_DATA_LEN: usize = MAX_PKT_LINE_LEN - PKT_LINE_SIZE_LEN;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PktLine<T> {
    Flush,
    Line(T),
}

pub fn write<W: io::Write, B: AsRef<[u8]>>(mut writer: W, buf: B) -> io::Result<()> {
    let buf = buf.as_ref();
    if buf.is_empty() {
        return Ok(());
    }
    if buf.len() > MAX_PKT_LINE_DATA_LEN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "The maximum length of a pkt-line's data component is 65516 bytes."));
    }
    write!(writer, "{:04x}", buf.len() + PKT_LINE_SIZE_LEN)?;
    writer.write_all(buf.as_ref())
}

pub fn write_str<W: io::Write, S: AsRef<str>>(mut writer: W, buf: S) -> io::Result<()> {
    let buf = buf.as_ref().as_bytes();
    if buf.is_empty() {
        return Ok(());
    }
    let len = buf.len();
    if buf.len() > MAX_PKT_LINE_DATA_LEN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "The maximum length of a pkt-line's data component is 65516 bytes."));
    }
    let inject_line_feed = buf[buf.len() - 1] != b'\n' && buf.len() < MAX_PKT_LINE_DATA_LEN;
    write!(writer, "{:04x}", len + if inject_line_feed { 5 } else { 4 })?;
    writer.write_all(buf)?;
    if inject_line_feed {
        writer.write_all(b"\n")?;
    }
    Ok(())
}

pub fn flush<W: io::Write>(mut writer: W) -> io::Result<()> {
    writer.write_all(b"0000")
}

fn read_pkt_line_size<R: io::Read>(mut reader: R) -> io::Result<PktLine<usize>> {
    let mut size_buf = [0; PKT_LINE_SIZE_LEN];
    reader.read_exact(&mut size_buf)?;
    let size_str = str::from_utf8(&size_buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let size = usize::from_str_radix(size_str, 16)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if size == 0 {
        return Ok(PktLine::Flush);
    }
    if size < PKT_LINE_SIZE_LEN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Size less than 4 and not equal to 0"));
    }
    if size > MAX_PKT_LINE_LEN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Size greater than 65520"));
    }
    Ok(PktLine::Line(size - PKT_LINE_SIZE_LEN))
}

pub fn read<'a, R: io::Read>(mut reader: R, buf: &'a mut [u8; MAX_PKT_LINE_DATA_LEN]) -> io::Result<PktLine<&'a [u8]>> {
    let size = read_pkt_line_size(&mut reader)?;
    let size = match size {
        PktLine::Line(size) => size,
        PktLine::Flush => return Ok(PktLine::Flush),
    };
    reader.read_exact(&mut buf[..size])?;
    if size > 0 && buf[size - 1] == b'\n' {
        Ok(PktLine::Line(&buf[..size - 1]))
    } else {
        Ok(PktLine::Line(&buf[..size]))
    }
}

pub fn each<R: io::Read, F: FnMut(PktLine<&[u8]>) -> io::Result<()>>(mut reader: R, mut callback: F) -> io::Result<()> {
    let mut buffer = [0; MAX_PKT_LINE_DATA_LEN];
    loop {
        match read(&mut reader, &mut buffer) {
            Ok(line) => {
                callback(line)?;
            }
            Err(e) => match e.kind() {
                io::ErrorKind::UnexpectedEof => return Ok(()),
                _ => return Err(e),
            }
        }
    }
}

pub fn each_str<R: io::Read, F: FnMut(PktLine<&str>) -> io::Result<()>>(reader: R, mut callback: F) -> io::Result<()>{
    each(reader, |line| {
        callback(match line {
            PktLine::Flush => PktLine::Flush,
            PktLine::Line(buf) => {
                let line = str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                PktLine::Line(line)
            }
        })
    })
}
