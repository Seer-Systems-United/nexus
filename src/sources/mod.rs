//! # Polling sources module
//!
//! Handles ingestion, parsing, and caching of polling data from multiple
//! providers: Emerson, Gallup, Ipsos, and YouGov.
//!
//! ## Module structure
//!
//! - `data`: Common data types for polling source responses.
//! - `date`: Date handling utilities for source data.
//! - `emerson`: Emerson College polling source implementation.
//! - `gallup`: Gallup polling source implementation.
//! - `ipsos`: Ipsos polling source implementation.
//! - `persistance`: Caching and refresh logic for source data.
//! - `registry`: Source registry and identification.
//! - `scope`: Query scope handling (latest, last N days/weeks/months).
//! - `yougov`: YouGov polling source implementation.
//!
//! ## Source trait
//!
//! All sources implement the `Source` async trait, which defines a
//! `get_data` method for fetching data with a given `Scope`.

pub mod data;
pub mod date;
pub mod emerson;
pub mod gallup;
pub mod ipsos;
pub mod persistance;
pub mod registry;
pub mod scope;
pub mod yougov;

use std::error::Error;

pub use data::*;
pub use registry::SourceId;
pub use scope::Scope;

/// Async trait implemented by all polling source providers.
///
/// Each source must define a constant `NAME` and implement `get_data`
/// to fetch polling data for a given query scope.
///
/// # Associated constants
///
/// - `NAME`: Human-readable source name (e.g., "emerson", "gallup").
/// - `CACHE_VERSION`: Cache version string (defaults to "v1").
#[async_trait::async_trait]
pub trait Source {
    const NAME: &'static str;
    const CACHE_VERSION: &'static str = "v1";

    /// Fetches polling data for the given query scope.
    ///
    /// # Parameters
    ///
    /// - `scope`: The query scope (latest, last N days/weeks/months, etc.).
    ///
    /// # Returns
    ///
    /// Returns `Ok(DataCollection)` with the fetched polling data.
    /// Returns `Err` if fetching, parsing, or validation fails.
    async fn get_data(scope: Scope) -> Result<DataCollection, Box<dyn Error + Send + Sync>>;
}
