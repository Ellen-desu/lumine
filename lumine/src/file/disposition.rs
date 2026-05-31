//! HTTP Content-Disposition values.
//!
//! This module defines the [`Disposition`] enum, which represents how a file
//! should be handled by the client (browser).

/// Represents the `Content-Disposition` of a file.
#[derive(Debug, Clone, PartialEq)]
pub enum Disposition {
    /// The file should be displayed within the browser if possible.
    Inline,
    /// The file should be downloaded by the browser.
    Attachment,
}
