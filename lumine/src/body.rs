use crate::{attachment::Attachment, stream::Stream};

pub enum Body<S: Stream> {
    Empty,
    Bytes(Vec<u8>),

    // Chunked transfer encoding
    Attachment(Attachment),
    Chunked(S),
}
