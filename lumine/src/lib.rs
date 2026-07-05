#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Ellen-desu/lumine/refs/heads/main/assets/lumine.png",
    html_favicon_url = "https://raw.githubusercontent.com/Ellen-desu/lumine/refs/heads/main/assets/lumine.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

pub mod application;
pub mod body;
pub mod error;
pub mod middleware;
pub mod prelude;
pub mod request;
pub mod response;
pub mod stream;

#[doc(hidden)]
pub mod internal;

#[doc(hidden)]
pub mod routing;

#[cfg(feature = "filestream")]
pub mod filestream;

#[doc(inline)]
pub use crate::{
    application::{Limits, Lumine, Timeouts},
    body::{Body, IntoBody},
    error::Error,
    middleware::{Middleware, Next},
    request::{FromRequest, Params, Query, Request},
    response::{IntoResponse, Response},
    stream::Stream,
};

#[doc(inline)]
#[cfg(feature = "filestream")]
pub use filestream::{Disposition, FileStream};
