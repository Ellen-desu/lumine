//! The Lumine prelude.
//!
//! This module re-exports common types, traits, and modules necessary
//! to build applications with Lumine.
//!
//! Users are encouraged to import this module via `use lumine::prelude::*;`
//! for a convenient development experience.

#[doc(no_inline)]
pub use crate::{
    Body, Error, IntoBody, IntoResponse, Limits, Lumine, Middleware, Next, Params, Path, Query,
    Request, Response, Stream, Timeouts,
};

#[doc(no_inline)]
pub use http::{
    self, HeaderMap, HeaderName, HeaderValue, Method, StatusCode, Uri, Version, header,
};

#[doc(no_inline)]
#[cfg(feature = "filestream")]
pub use crate::{Disposition, FileStream};

#[doc(no_inline)]
#[cfg(feature = "tls")]
pub use crate::TlsExt;
