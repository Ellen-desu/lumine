//! File streaming and attachment handling.
//!
//! This module provides the [`FileStream`] struct, which facilitates sending files
//! as response bodies. It handles file opening, metadata retrieval (like length),
//! and provides automatic MIME type inference using the `infer` crate.
//!
//! `FileStream` implements the [`Stream`](crate::stream::Stream) trait, allowing
//! it to be used directly in a [`Body::Stream`](crate::body::Body::Stream).

pub mod disposition;

#[doc(inline)]
pub use self::disposition::Disposition;

use http::{HeaderMap, header};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};
use tokio::{fs::File, io::BufReader};

/// A file stream for HTTP responses.
///
/// `FileStream` manages a file handle and its metadata, allowing it to be
/// streamed as a response body. It automatically detects the MIME type
/// and provides file metadata like size and name.
#[derive(Debug)]
pub struct FileStream {
    pub(crate) reader: BufReader<File>,
    pub(crate) headers: HeaderMap,
    pub(crate) length: usize,
}

impl FileStream {
    /// Opens a file at the given path with a specific [`Disposition`].
    ///
    /// This method will:
    /// 1. Open the file.
    /// 2. Extract the filename.
    /// 3. Retrieve file metadata (length).
    /// 4. Infer the MIME type.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if metadata cannot be retrieved.
    pub async fn open_with_disposition(
        path: impl AsRef<Path>,
        disposition: Disposition,
    ) -> std::io::Result<Self> {
        let mut headers = HeaderMap::new();
        let path = path.as_ref();

        let file = File::open(path).await?;

        let filename = {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                name.chars()
                    .filter(|c| !c.is_control())
                    .collect::<String>()
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
            } else {
                "download".to_string()
            }
        };

        let length = file.metadata().await?.len() as usize;

        let reader = BufReader::new(file);

        let mime_type = match infer::get_from_path(path)? {
            Some(info) => info.mime_type(),
            None => "application/octet-stream",
        };

        headers.insert(
            header::CONTENT_TYPE,
            mime_type
                .parse()
                .expect("parsing from str to header value should never fail"),
        );

        headers.insert(
            header::CONTENT_DISPOSITION,
            format!(
                "{}; filename=\"{}\"",
                match disposition {
                    Disposition::Attachment => "attachment",
                    Disposition::Inline => "inline",
                },
                filename
            )
            .parse()
            .expect("parsing from str to header value should never fail"),
        );

        Ok(Self {
            reader,
            headers,
            length,
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
