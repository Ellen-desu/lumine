use crate::types::body::Body;

/// The response type produced by handlers and the server runtime.
///
/// This is an alias for `http::Response<Body>`, representing a
/// fully constructed HTTP response with a byte-oriented body.
///
/// All values returned from handlers are ultimately converted
/// into this type via the `IntoResponse` trait.
pub type Response = http::Response<Body>;
