//! Dynamic response body type.
//!
//! This module defines the [`DynBody`] type alias, which represents a
//! response body that can be any type implementing the [`Stream`] trait.
//! It uses a trait object (`Box<dyn Stream>`) to allow for heterogeneous
//! streaming types.

use crate::{body::Body, stream::Stream};

/// A type-erased response body.
///
/// `DynBody` is a [`Body`] that holds a boxed [`Stream`] trait object.
/// This allows handlers to return different types of streams in a single
/// response type.
pub type DynBody = Body<Box<dyn Stream>>;
