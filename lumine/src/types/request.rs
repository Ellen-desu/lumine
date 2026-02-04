use crate::types::body::Body;

/// The request type used throughout Lumine.
///
/// This is an alias for `http::Request<Body>`, binding the
/// request body type to Lumine's [`Body`] definition.
///
/// Using a type alias keeps function signatures concise and
/// ensures consistent request handling across the codebase.
pub type Request = http::Request<Body>;
