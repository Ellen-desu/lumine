use crate::types::result::Result;

pub trait Stream {
    fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize>;
}
