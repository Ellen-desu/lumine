//! Application core and type-state system.
//!
//! This module defines the core structures of Lumine, including:
//!
//! - [`Lumine`], the main HTTP application entry point
//! - [`Client`], the request/client context returned once the server is ready
//! - The compile-time state system used to enforce correct lifecycle usage
//!
//! ---
//!
//! ## Type-State Pattern
//!
//! Lumine uses a **type-state pattern** to represent lifecycle phases
//! at compile time.
//!
//! Instead of tracking states through runtime flags or booleans,
//! Lumine encodes state directly into the type system using generics.
//!
//! This means invalid operations are rejected by the compiler,
//! not discovered later at runtime.
//!
//! ---
//!
//! ## Core Lifecycle States
//!
//! Lumine defines two primary marker states:
//!
//! - [`Builder`] — configuration phase
//!   Routes, middleware, and server settings may still be modified.
//!
//! - [`Ready`] — runtime phase
//!   Configuration is finalized and the server is safe to serve requests.
//!
//! ---
//!
//! ## Application Lifecycle (`Lumine`)
//!
//! A new application always starts in the [`Builder`] state:
//!
//! ```rust
//! use lumine::Lumine;
//!
//! let app = Lumine::builder();
//! ```
//!
//! Calling [`Lumine::build`] finalizes the configuration and transitions
//! into the [`Ready`] state:
//!
//! ```rust
//! use lumine::Lumine;
//!
//! let app = Lumine::builder()
//!     .route("/", |_| "Hello, World!")
//!     .build();
//! ```
//!
//! Only applications in the [`Ready`] state can be served.
//!
//! ---
//!
//! ## Client Lifecycle (`Client`)
//!
//! Lumine also applies the same type-state concept to request/client metadata.
//!
//! A [`Client`] begins in a [`Builder`] state while request information
//! is still being assembled internally.
//!
//! Once ready, it transitions into [`Client<Ready>`], where metadata becomes
//! immutable and can be safely accessed through getter methods.
//!
//! This ensures request context cannot be mutated once the server begins
//! handling it.
//!
//! ---
//!
//! ## Design Rationale
//!
//! By encoding lifecycle phases into the type system, Lumine provides:.map(|b| b.as_ref())
//!
//! - Clear separation between configuration and runtime phases
//! - Compile-time guarantees about correct API usage
//! - A predictable and ergonomic mental model
//! - Zero runtime overhead (thanks to marker types + `PhantomData`)
//!
//! Internally, Lumine uses marker types and `PhantomData` to associate
//! state information without affecting performance.

pub mod client;
pub mod lumine;
pub mod states;

pub use self::{
    client::Client,
    lumine::Lumine,
    states::{Builder, Ready},
};
