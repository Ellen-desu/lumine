/// The concrete body type used by Lumine.
///
/// `Body` represents the raw bytes of an HTTP message body.
/// It is currently defined as a `Vec<u8>`, reflecting the
/// byte-oriented nature of HTTP.
///
/// This alias exists to centralize the body type and allow
/// future changes without affecting public APIs.
pub type Body = Vec<u8>;
