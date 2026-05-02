//! # YouGov polling source
//!
//! Defines the `YouGov` source type and re-exports server modules.

pub mod server;

/// YouGov Poll source identifier.
///
/// Implements `Source` trait via `server::load_yougov`.
#[derive(Debug, Clone, Default)]
pub struct YouGov;
