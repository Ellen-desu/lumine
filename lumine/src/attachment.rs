//! File attachment and streaming.
//!
//! This module provides the [`Attachment`] struct, which facilitates sending files
//! as response bodies. It handles file opening, metadata retrieval (like length),
//! and provides automatic MIME type inference using the `infer` crate.
//!
//! Attachments implement the [`Stream`] trait, allowing
//! them to be used directly in a [`Body::Stream`](crate::body::Body::Stream).

use infer::Type;
use std::{
    fs::File,
    io::{BufReader, Read},
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{stream::Stream, types::result::Result};

/// A file attachment for HTTP responses.
///
/// `Attachment` manages a file handle and its metadata, allowing it to be
/// streamed as a response body.
#[derive(Debug)]
pub struct Attachment {
    pub(crate) reader: BufReader<File>,
    /// The name of the file to be sent in the `Content-Disposition` header.
    pub(crate) filename: &'static str,
    /// The total size of the file in bytes.
    pub(crate) length: usize,
    /// Inferred file type information (MIME type, etc.).
    pub(crate) info: Option<Type>,
}

impl Attachment {
    /// Opens a file at the specified path and prepares it as an attachment.
    ///
    /// This method will:
    /// 1. Open the file.
    /// 2. Retrieve the file length.
    /// 3. Attempt to infer the MIME type based on the file content.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::error::Error) if the file cannot be opened or metadata
    /// cannot be retrieved.
    pub fn open(path: impl AsRef<Path>, filename: &'static str) -> Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)?;
        let length = file.metadata()?.len() as usize;
        let reader = BufReader::new(file);
        let info = infer::get_from_path(path)?;

        Ok(Self {
            reader,
            filename,
            length,
            info,
        })
    }
}

impl Deref for Attachment {
    type Target = BufReader<File>;

    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}

impl DerefMut for Attachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reader
    }
}

impl Stream for Attachment {
    fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buffer)?)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.length)
    }
}
