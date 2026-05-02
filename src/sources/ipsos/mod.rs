//! # Ipsos polling source
//!
//! Defines the `Ipsos` source type and re-exports server modules.

pub mod server;

/// Ipsos Poll source identifier.
///
/// Implements `Source` trait via `server::load_ipsos`.
#[derive(Debug, Clone, Default)]
pub struct Ipsos;
