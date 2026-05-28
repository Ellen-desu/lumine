use crate::{body::Body, stream::Stream};

pub type DynBody = Body<Box<dyn Stream>>;
