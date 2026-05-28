use infer::Type;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{stream::Stream, types::result::Result};

#[derive(Debug)]
pub struct Attachment {
    pub(crate) reader: BufReader<File>,
    pub(crate) filename: &'static str,
    pub(crate) length: usize,
    pub(crate) info: Option<Type>,
}

impl Attachment {
    pub fn open(path: impl AsRef<Path>, filename: &'static str) -> io::Result<Self> {
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
