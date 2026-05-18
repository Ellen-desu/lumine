#![doc = include_str!("../README.md")]

mod internal;

pub mod application;
pub mod error;
pub mod middleware;
pub mod routing;
pub mod types;

#[doc(inline)]
pub use crate::{
    application::Lumine,
    error::Error,
    middleware::{Middleware, Next},
    routing::{IntoResponse, Params, Path, Query, Route},
    types::{Body, Request, Response, Result},
};

#[cfg(feature = "bench")]
#[doc(hidden)]
pub use crate::internal::parser::{
    parse_body_for_bench, parse_headers_for_bench, parse_request_line_for_bench,
};

pub use http;
