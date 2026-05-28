/// The request type used throughout Lumine.
///
/// This is an alias for `http::Request<Vec<u8>>`, binding the
/// request body type to [`Vec<u8>`] definition.
///
/// Using a type alias keeps function signatures concise and
/// ensures consistent request handling across the codebase.
pub type Request = http::Request<Vec<u8>>;
