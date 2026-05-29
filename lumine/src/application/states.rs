//! Application lifecycle states.
//!
//! This module defines the marker types used in Lumine's type-state
//! pattern to track the application's configuration and runtime
//! phases at compile time.

/// Marker type indicating that the application is in the configuration phase.
///
/// In this state, routes and server settings can be modified.
pub struct Building;

/// Marker type indicating that the application is ready to run.
///
/// In this state, the server configuration is finalized and the application
/// can be served.
pub struct Ready;
