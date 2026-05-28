use crate::types::result::Result;

pub trait Stream {
    fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize>;

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<T: Stream + ?Sized> Stream for Box<T> {
    fn next_chunk(&mut self, buf: &mut [u8]) -> Result<usize> {
        (**self).next_chunk(buf)
    }
}
