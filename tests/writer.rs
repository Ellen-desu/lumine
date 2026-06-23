use std::io::Cursor;

use lumine::{internal::writer::write_response, prelude::*};
use tokio::io::AsyncReadExt;

struct StaticStream(Cursor<Vec<u8>>);

#[async_trait::async_trait]
impl Stream for StaticStream {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.get_ref().len())
    }
    fn headers_hint(&self) -> Option<&HeaderMap> {
        None
    }
    async fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.0.read(buffer).await
    }
}

struct ChunkedStream {
    chunks: Vec<Vec<u8>>,
    index: usize,
}

#[async_trait::async_trait]
impl Stream for ChunkedStream {
    async fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        let Some(chunk) = self.chunks.get(self.index) else {
            return Ok(0);
        };

        buffer[..chunk.len()].copy_from_slice(chunk);
        self.index += 1;
        Ok(chunk.len())
    }
}

#[tokio::test]
async fn body_bytes() {
    let response = Response::new(Body::Bytes("Hello, World!".as_bytes().to_vec()));
    let mut cursor = Cursor::new(Vec::new());
    let timeouts = Timeouts::default();

    assert!(
        write_response(response, &mut cursor, &timeouts)
            .await
            .is_ok()
    );

    assert_eq!(cursor.into_inner(), b"HTTP/1.1 200 OK\r\n\r\nHello, World!");
}

#[tokio::test]
async fn body_empty() {
    let response = Response::new(Body::Bytes(Vec::new()));
    let mut cursor = Cursor::new(Vec::new());
    let timeouts = Timeouts::default();

    assert!(
        write_response(response, &mut cursor, &timeouts)
            .await
            .is_ok()
    );

    assert_eq!(cursor.into_inner(), b"HTTP/1.1 200 OK\r\n\r\n");
}

#[tokio::test]
async fn body_static_stream() {
    let response = Response::new(Body::Stream(Box::new(StaticStream(Cursor::new(
        "Hello, World!".as_bytes().to_vec(),
    )))));
    let mut cursor = Cursor::new(Vec::new());
    let timeouts = Timeouts::default();

    assert!(
        write_response(response, &mut cursor, &timeouts)
            .await
            .is_ok()
    );

    assert_eq!(cursor.into_inner(), b"HTTP/1.1 200 OK\r\n\r\nHello, World!");
}

#[tokio::test]
async fn body_chunked_stream() {
    let response = Response::new(Body::Stream(Box::new(ChunkedStream {
        chunks: vec!["Hello, ".as_bytes().to_vec(), "World!".as_bytes().to_vec()],
        index: 0,
    })));
    let mut cursor = Cursor::new(Vec::new());
    let timeouts = Timeouts::default();

    assert!(
        write_response(response, &mut cursor, &timeouts)
            .await
            .is_ok()
    );

    let expected = b"HTTP/1.1 200 OK\r\n\r\n7\r\nHello, \r\n6\r\nWorld!\r\n0\r\n\r\n";

    assert_eq!(cursor.into_inner(), expected);
}
