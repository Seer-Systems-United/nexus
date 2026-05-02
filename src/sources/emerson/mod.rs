//! # Emerson polling source
//!
//! Defines the `Emerson` source type and re-exports server modules.

pub mod server;

/// Emerson College Polling source identifier.
///
/// Implements `Source` trait via `server::load_emerson`.
#[derive(Debug, Clone, Default)]
pub struct Emerson;
