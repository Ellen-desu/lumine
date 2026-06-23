//! Internal implementation module.
//!
//! This module contains internal implementation details for handling requests,
//! parsing HTTP, and managing connections. It is not intended for public use.

pub mod connection;
pub mod dispatch;
pub mod framing;
pub mod parser;
pub mod reader;
pub mod validator;
pub mod writer;
