//! Application core and state system.
//!
//! This module defines [`Lumine`], the main HTTP application structure,
//! along with its compile-time state system.
//!
//! ## Application Lifecycle
//!
//! Lumine uses a **type-state pattern** to represent the application
//! lifecycle at compile time. Instead of tracking state at runtime,
//! the application state is encoded in the type system.
//!
//! The lifecycle consists of two primary states:
//!
//! - [`Builder`] — configuration phase
//!   Routes and server settings can be added or modified.
//!
//! - [`Ready`] — runtime phase
//!   Configuration is finalized and the application can be served.
//!
//! This design ensures that invalid operations (such as modifying routes
//! after the server has started) are prevented at compile time rather than
//! at runtime.
//!
//! ## State Transitions
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
//! the application into the [`Ready`] state:
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
//! ## Design Rationale
//!
//! By encoding the application lifecycle in the type system, Lumine provides:
//!
//! - Clear separation between configuration and runtime phases
//! - Compile-time guarantees about correct API usage
//! - A simpler and more predictable mental model for users
//!
//! Internally, marker types and `PhantomData` are used to associate
//! state information without introducing runtime overhead.

pub mod lumine;
pub mod states;

pub use lumine::Lumine;
pub use states::{Builder, Ready};
