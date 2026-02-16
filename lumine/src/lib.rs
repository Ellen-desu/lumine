#![doc = include_str!("../README.md")]

mod internal;

pub mod application;
pub mod error;
pub mod routing;
pub mod traits;
pub mod types;

#[doc(inline)]
pub use crate::application::lumine::Lumine;

#[doc(inline)]
pub use crate::error::Error;

#[doc(inline)]
pub use crate::routing::{params::Params, path::Path, query::Query};

#[doc(inline)]
pub use crate::traits::{into_body::IntoBody, into_response::IntoResponse};

#[doc(inline)]
pub use crate::types::{body::Body, request::Request, response::Response, result::Result};

pub use http;
