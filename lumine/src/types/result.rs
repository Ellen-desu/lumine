use crate::error::Error;

/// The standard result type used throughout Lumine.
///
/// This alias binds the error type to Lumine's internal [`Error`],
/// reducing repetition and ensuring consistent error handling
/// across modules.
pub type Result<T> = std::result::Result<T, Error>;
