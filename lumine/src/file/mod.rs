//! File handling utilities.
//!
//! This module provides tools for working with files in the context of HTTP
//! responses, including streaming file content and managing content disposition.

pub mod disposition;
pub mod filestream;

pub use self::{disposition::Disposition, filestream::FileStream};
