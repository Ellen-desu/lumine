//! Response writing module.
//!
//! This module provides functionality for writing HTTP responses to a TCP stream,
//! supporting both static and chunked body writing.

use crate::{application::Timeouts, body::Body, response::Response, stream::Stream};
use bytes::{BufMut, BytesMut};
use std::io::{Cursor, Write};
use std::time::Duration;
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Writes an HTTP response to the provided TCP stream.
///
/// This function writes the status line, headers, and body of the response
/// to the stream using a buffered writer.
pub async fn write_response<W: AsyncWrite + Unpin>(
    response: Response,
    stream: &mut W,
    timeouts: &Timeouts,
) -> std::io::Result<()> {
    let mut buffer = BytesMut::with_capacity(512);

    let status = response.status();

    buffer.put_slice(b"HTTP/1.1 ");
    buffer.put_slice(status.as_str().as_bytes());
    buffer.put_u8(b' ');
    buffer.put_slice(status.canonical_reason().unwrap_or("Unknown").as_bytes());
    buffer.put_slice(b"\r\n");

    // Headers
    for (name, value) in response.headers() {
        buffer.put_slice(name.as_str().as_bytes());
        buffer.put_slice(b": ");
        buffer.put_slice(value.as_bytes());
        buffer.put_slice(b"\r\n");
    }

    // End of headers
    buffer.put_slice(b"\r\n");

    tokio::time::timeout(timeouts.response_write, stream.write_all(&buffer)).await??;

    // Write body to the stream
    match response.into_body() {
        Body::Bytes(bytes) => {
            tokio::time::timeout(timeouts.response_write, stream.write_all(&bytes)).await??;
        }
        Body::Stream(mut bytes_stream) => {
            let mut buffer = [0u8; 8192];

            if bytes_stream.size_hint().is_some() {
                write_body_static(stream, &mut buffer, &mut bytes_stream, timeouts).await?;
            } else {
                write_body_chunked(stream, &mut buffer, &mut bytes_stream, timeouts).await?;
            }
        }
        _ => {}
    };

    tokio::time::timeout(timeouts.response_write, stream.flush()).await??;

    Ok(())
}

/// Reads the next chunk from a stream with a timeout.
///
/// Returns `Some(n)` with the number of bytes read, or `None` on end-of-stream,
/// error, or timeout.
async fn read_next_chunk<S: Stream>(
    stream: &mut S,
    buffer: &mut [u8],
    timeout: Duration,
) -> Option<usize> {
    match tokio::time::timeout(timeout, stream.next_chunk(buffer)).await {
        Ok(Ok(0)) => None,
        Ok(Ok(n)) => Some(n),
        _ => None,
    }
}

/// Writes data to the writer with a timeout.
///
/// Returns `true` on success, `false` on timeout or I/O error.
async fn write_timed<W: AsyncWrite + Unpin>(
    writer: &mut W,
    data: &[u8],
    timeout: Duration,
) -> bool {
    tokio::time::timeout(timeout, writer.write_all(data))
        .await
        .is_ok()
}

/// Writes a streaming body to the writer using chunked transfer encoding.
pub async fn write_body_chunked<S: Stream, W: AsyncWrite + Unpin>(
    writer: &mut W,
    buffer: &mut [u8],
    bytes_stream: &mut S,
    timeouts: &Timeouts,
) -> std::io::Result<()> {
    let mut chunk_header = [0u8; 32];

    while let Some(n) = read_next_chunk(bytes_stream, buffer, timeouts.stream_read).await {
        let len = {
            let mut cursor = Cursor::new(&mut chunk_header[..]);
            write!(cursor, "{:x}\r\n", n)?;
            cursor.position() as usize
        };

        if !write_timed(writer, &chunk_header[..len], timeouts.response_write).await
            || !write_timed(writer, &buffer[..n], timeouts.response_write).await
            || !write_timed(writer, b"\r\n", timeouts.response_write).await
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
    while let Some(n) = read_next_chunk(bytes_stream, buffer, timeouts.stream_read).await {
        if !write_timed(writer, &buffer[..n], timeouts.response_write).await {
            break;
        }
    }

    Ok(())
}
