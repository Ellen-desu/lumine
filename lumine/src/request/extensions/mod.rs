//! Request extensions provided by Lumine.
//!
//! This module groups the strongly-typed values that Lumine attaches to every
//! incoming [`Request`](crate::request::Request) before a handler is invoked.
//! They are stored in the request's [extension map](http::Extensions) and can
//! be retrieved through the [`FromRequest`](crate::request::FromRequest) trait.
//!
//! # Available extensions
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Addr`] | The remote IP address of the connected client. |
//! | [`Params`] | Dynamic path segments extracted during route matching. |
//! | [`Query`] | Query-string key/value pairs parsed from the request URI. |
//! | [`Remainder`] | The remaining path after wildcard segments have been matched. |
//!
//! All four types are re-exported at the crate root for convenience, so you
//! generally do not need to reference this module directly.

pub mod addr;
pub mod params;
pub mod query;
pub mod remainder;

#[doc(inline)]
pub use self::{addr::Addr, params::Params, query::Query, remainder::Remainder};
