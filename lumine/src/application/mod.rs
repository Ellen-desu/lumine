//! Application core and type-state system.
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
//! - [`Building`] — configuration phase
//!   Routes, middleware, and server settings may still be modified.
//!
//! - [`Ready`] — runtime phase
//!   Configuration is finalized and the server is safe to serve requests.
//!
//! ---
//!
//! ## Application Lifecycle (`Lumine`)
//!
//! A new application always starts in the [`Building`] state:
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
//!     .route("/", async |_| "Hello, World!")
//!     .build();
//! ```
//!
//! Only applications in the [`Ready`] state can be served.
//!
//! ---
//!
//! ## Design Rationale
//!
//! By encoding lifecycle phases into the type system, Lumine provides:
//!
//! - Clear separation between configuration and runtime phases
//! - Compile-time guarantees about correct API usage
//! - A predictable and ergonomic mental model
//! - Zero runtime overhead (thanks to marker types + `PhantomData`)
//!
//! Internally, Lumine uses marker types and `PhantomData` to associate
//! state information without affecting performance.

pub mod limits;
pub mod lumine;
pub mod states;
pub mod timeouts;

#[doc(inline)]
pub use self::{
    limits::Limits,
    lumine::Lumine,
    states::{Building, Ready},
    timeouts::Timeouts,
};
