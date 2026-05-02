//! # Topic data types
//!
//! Re-exports and aggregates type modules for polling topics:
//! demographics, observations, and topic summaries.

mod demographics;
mod observations;
mod topics;

pub use crate::sources::SourceId;
pub use demographics::*;
pub use observations::*;
pub use topics::*;
