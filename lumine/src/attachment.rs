use infer::Type;
use std::{
    fs::File,
    io,
    ops::{Deref, DerefMut},
    path::Path,
};

#[derive(Debug)]
pub struct Attachment {
    pub(crate) file: File,
    pub(crate) filename: &'static str,
    pub(crate) info: Option<Type>,
}

impl Attachment {
    pub fn open(path: impl AsRef<Path>, filename: &'static str) -> io::Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)?;
        let info = infer::get_from_path(path)?;

        Ok(Self {
            file,
            filename,
            info,
        })
    }
}

impl Deref for Attachment {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for Attachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}
