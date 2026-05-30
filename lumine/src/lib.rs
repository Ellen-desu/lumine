#![doc = include_str!("../README.md")]

mod internal;

pub mod application;
pub mod body;
pub mod error;
pub mod file;
pub mod middleware;
pub mod routing;
pub mod stream;
pub mod types;
pub mod utils;

#[doc(inline)]
pub use crate::{
    application::Lumine,
    body::Body,
    error::Error,
    file::{Disposition, FileStream},
    middleware::{Middleware, Next},
    routing::{IntoResponse, Params, Path, Query, Route},
    types::{Request, Response, Result},
    utils::SetHeaders,
};

#[cfg(feature = "bench")]
#[doc(hidden)]
pub use crate::internal::parser::{
    parse_body_for_bench, parse_headers_for_bench, parse_request_line_for_bench,
};

pub use http;
