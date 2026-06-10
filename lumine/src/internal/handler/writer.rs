use crate::{
    body::Body,
    stream::Stream,
    types::{response::Response, result::Result},
};
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
};

pub fn write_response(response: Response, stream: &TcpStream) -> Result<()> {
    let mut writer = BufWriter::new(stream);

    // Status line
    let version = response.version();
    let status = response.status();

    // Write the status line to the stream
    write!(writer, "{version:?} {status}\r\n")?;

    // Headers
    for (name, value) in response.headers() {
        // Write each header to the stream
        write!(writer, "{name}: {}\r\n", value.to_str().unwrap_or_default())?;
    }

    // End of headers
    writer.write_all(b"\r\n")?;

    // Write body to the stream
    match response.into_body() {
        Body::Bytes(bytes) => {
            writer.write_all(&bytes)?;
        }
        Body::Stream(stream) => {
            if stream.size_hint().is_some() {
                write_body_static(stream, &mut writer)?;
            } else {
                write_body_chunked(stream, &mut writer)?;
            }
        }
        _ => {}
    };

    writer.flush()?;

    Ok(())
}

pub fn write_body_chunked<S: Stream, W: Write>(mut bytes_stream: S, writer: &mut W) -> Result<()> {
    let mut buffer = [0u8; 8192];

    loop {
        let n = bytes_stream.next_chunk(&mut buffer)?;
        if n == 0 {
            break;
        }

        write!(writer, "{n:x}\r\n")?;
        writer.write_all(&buffer[..n])?;
        writer.write_all(b"\r\n")?;
    }

    writer.write_all(b"0\r\n\r\n")?;

    Ok(())
}

pub fn write_body_static<S: Stream, W: Write>(mut bytes_stream: S, writer: &mut W) -> Result<()> {
    let mut buffer = [0u8; 8192];

    loop {
        let n = bytes_stream.next_chunk(&mut buffer)?;
        if n == 0 {
            break;
        }

        writer.write_all(&buffer[..n])?;
    }

    Ok(())
}
