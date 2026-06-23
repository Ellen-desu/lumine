//! Response writing module.
//!
//! This module provides functionality for writing HTTP responses to a TCP stream,
//! supporting both static and chunked body writing.

use crate::{application::Timeouts, body::Body, response::Response, stream::Stream};
use std::io::Write;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

/// Writes an HTTP response to the provided TCP stream.
///
/// This function writes the status line, headers, and body of the response
/// to the stream using a buffered writer.
pub async fn write_response<W: AsyncWrite + Unpin>(
    response: Response,
    stream: &mut W,
    timeouts: &Timeouts,
) -> std::io::Result<()> {
    let mut writer = BufWriter::new(stream);
    let mut buffer = Vec::with_capacity(512);

    // Status line
    let version = response.version();
    let status = response.status();

    // Write the status line to the stream
    write!(buffer, "{version:?} {status}\r\n")?;

    // Headers
    for (name, value) in response.headers() {
        // Write each header to the stream
        write!(buffer, "{name}: {}\r\n", value.to_str().unwrap_or_default())?;
    }

    // End of headers
    buffer.extend_from_slice(b"\r\n");

    tokio::time::timeout(timeouts.response_write, writer.write_all(&buffer)).await??;

    // Write body to the stream
    match response.into_body() {
        Body::Bytes(bytes) => {
            tokio::time::timeout(timeouts.response_write, writer.write_all(&bytes)).await??;
        }
        Body::Stream(mut bytes_stream) => {
            let mut buffer = [0u8; 8192];

            if bytes_stream.size_hint().is_some() {
                write_body_static(&mut writer, &mut buffer, &mut bytes_stream, timeouts).await?;
            } else {
                write_body_chunked(&mut writer, &mut buffer, &mut bytes_stream, timeouts).await?;
            }
        }
        _ => {}
    };

    tokio::time::timeout(timeouts.response_write, writer.flush()).await??;

    Ok(())
}

/// Writes a streaming body to the writer using chunked transfer encoding.
pub async fn write_body_chunked<S: Stream, W: AsyncWrite + Unpin>(
    writer: &mut W,
    buffer: &mut [u8],
    bytes_stream: &mut S,
    timeouts: &Timeouts,
) -> std::io::Result<()> {
    let mut chunk_header = [0u8; 32];

    loop {
        let n = match tokio::time::timeout(timeouts.stream_read, bytes_stream.next_chunk(buffer))
            .await
        {
            Ok(Ok(0)) => break,
            Ok(Ok(n)) => n,
            _ => break,
        };

        let len = {
            let mut cursor = std::io::Cursor::new(&mut chunk_header[..]);
            write!(cursor, "{:x}\r\n", n)?;
            cursor.position() as usize
        };

        if tokio::time::timeout(
            timeouts.response_write,
            writer.write_all(&chunk_header[..len]),
        )
        .await
        .is_err()
        {
            break;
        }

        if tokio::time::timeout(timeouts.response_write, writer.write_all(&buffer[..n]))
            .await
            .is_err()
        {
            break;
        }

        if tokio::time::timeout(timeouts.response_write, writer.write_all(b"\r\n"))
            .await
            .is_err()
        {
            break;
        }
    }

    tokio::time::timeout(timeouts.response_write, writer.write_all(b"0\r\n\r\n")).await??;

    Ok(())
}

/// Writes a streaming body to the writer directly (static size).
pub async fn write_body_static<S: Stream, W: AsyncWrite + Unpin>(
    writer: &mut W,
    buffer: &mut [u8],
    bytes_stream: &mut S,
    timeouts: &Timeouts,
) -> std::io::Result<()> {
    loop {
        let n = match tokio::time::timeout(timeouts.stream_read, bytes_stream.next_chunk(buffer))
            .await
        {
            Ok(Ok(0)) => break,
            Ok(Ok(n)) => n,
            _ => break,
        };

        if tokio::time::timeout(timeouts.response_write, writer.write_all(&buffer[..n]))
            .await
            .is_err()
        {
            break;
        }
    }

    Ok(())
}
