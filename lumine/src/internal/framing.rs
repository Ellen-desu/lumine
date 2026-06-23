#[derive(Debug, Clone, PartialEq)]
pub struct Framing {
    pub content_length: Option<usize>,
    pub connection: Connection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Connection {
    KeepAlive,
    Close,
}

impl Connection {
    pub fn is_keep_alive(&self) -> bool {
        matches!(self, Connection::KeepAlive)
    }

    pub fn is_close(&self) -> bool {
        matches!(self, Connection::Close)
    }
}
