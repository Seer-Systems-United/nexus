//! # Gallup polling source
//!
//! Defines the `Gallup` source type and re-exports server modules.

pub mod server;

/// Gallup Poll source identifier.
///
/// Implements `Source` trait via `server::load_gallup`.
#[derive(Debug, Clone, Default)]
pub struct Gallup;
