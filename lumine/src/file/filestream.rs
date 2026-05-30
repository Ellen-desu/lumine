//! File attachment and streaming.
//!
//! This module provides the [`Attachment`] struct, which facilitates sending files
//! as response bodies. It handles file opening, metadata retrieval (like length),
//! and provides automatic MIME type inference using the `infer` crate.
//!
//! Attachments implement the [`Stream`] trait, allowing
//! them to be used directly in a [`Body::Stream`](crate::body::Body::Stream).

use crate::{file::disposition::Disposition, stream::Stream, types::result::Result};
use std::{
    fs::File,
    io::{BufReader, Read},
    ops::{Deref, DerefMut},
    path::Path,
};

/// A file attachment for HTTP responses.
///
/// `Attachment` manages a file handle and its metadata, allowing it to be
/// streamed as a response body.
#[derive(Debug)]
pub struct FileStream {
    pub(crate) reader: BufReader<File>,
    /// The name of the file to be sent in the `Content-Disposition` header.
    pub(crate) filename: String,
    /// The total size of the file in bytes.
    pub(crate) length: usize,
    /// Inferred file type information (MIME type, etc.).
    pub(crate) mime_type: &'static str,
    /// The disposition type for the `Content-Disposition` header.
    pub(crate) disposition: Disposition,
}

impl FileStream {
    pub fn open_with_disposition(path: impl AsRef<Path>, disposition: Disposition) -> Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)?;

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("download")
            .to_string();

        let length = file.metadata()?.len() as usize;

        let reader = BufReader::new(file);

        let mime_type = match infer::get_from_path(path)? {
            Some(info) => info.mime_type(),
            None => "application/octet-stream",
        };

        Ok(Self {
            reader,
            filename,
            length,
            mime_type,
            disposition,
        })
    }
}

impl Deref for FileStream {
    type Target = BufReader<File>;

    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}

impl DerefMut for FileStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reader
    }
}

impl Stream for FileStream {
    fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buffer)?)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.length)
    }
}
