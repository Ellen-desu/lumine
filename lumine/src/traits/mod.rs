//! Core behavioral traits used across Lumine.
//!
//! This module defines a set of traits that describe **how values and
//! components behave**, rather than what data they contain.
//!
//! These traits form the connective tissue between routing, request
//! handling, and the HTTP layer.
//!
//! # Overview
//!
//! Lumine relies on small, focused traits to keep responsibilities
//! clearly separated:
//!
//! - [`RouteService`] defines how a route matches a path and dispatches
//!   a request to a handler.
//! - [`IntoResponse`] defines how a handler's return value is converted
//!   into an HTTP response.
//! - [`IntoBody`] defines how values are converted into raw bytes
//!   suitable for use as an HTTP body.
//!
//! Together, these traits allow Lumine to remain flexible while
//! maintaining a clear and predictable request–response pipeline.
//!
//! # Request–Response Flow
//!
//! At runtime, the interaction between these traits typically follows
//! this sequence:
//!
//! ```text
//! Incoming request
//!   ↓
//! Routing system
//!   ↓
//! RouteService::matches
//!   ↓
//! RouteService::call
//!   ↓
//! Handler return value
//!   ↓
//! IntoResponse::into_response
//!   ↓
//! HTTP Response
//!   ↓
//! Body conversion via IntoBody
//! ```
//!
//! Each trait is responsible for a single step in this pipeline,
//! preventing tight coupling between routing logic, handler code,
//! and HTTP encoding.
//!
//! # Design Principles
//!
//! - **Separation of concerns**
//!   Each trait addresses a single responsibility in the system.
//!
//! - **Explicit boundaries**
//!   Conversions between high-level application values and low-level
//!   HTTP representations are explicit and trait-driven.
//!
//! - **Extensibility**
//!   New behaviors can be introduced by implementing these traits
//!   without modifying existing routing or runtime code.
//!
//! This design keeps the core of Lumine small, understandable,
//! and easy to evolve over time.

pub mod into_body;
pub mod into_response;
pub mod route_service;

pub use into_body::IntoBody;
pub use into_response::IntoResponse;
pub use route_service::RouteService;
