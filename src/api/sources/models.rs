//! # Source API models
//!
//! Defines request/response structures for polling source endpoints
//! with OpenAPI schema support.

use crate::sources::SourceId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Summary representation of a polling source for API responses.
///
/// # Fields
/// - `id`: Static identifier for the source (e.g., "emerson", "gallup").
/// - `name`: Display name of the source.
#[derive(Debug, Serialize, ToSchema)]
pub struct SourceSummary {
    pub id: &'static str,
    pub name: &'static str,
}

/// Query parameters for source data requests.
///
/// # Fields
/// - `scope`: Optional scope string (e.g., "latest", "last_7_days").
/// - `count`: Optional count for scoped queries (alias: `n`).
/// - `n`: Alias for `count`.
/// - `question`: Optional question filter string.
#[derive(Debug, Deserialize)]
pub struct SourceQuery {
    pub scope: Option<String>,
    pub count: Option<u32>,
    pub n: Option<u32>,
    pub question: Option<String>,
}

/// Convert a `SourceId` into a `SourceSummary` for API responses.
impl From<SourceId> for SourceSummary {
    fn from(source: SourceId) -> Self {
        Self {
            id: source.id(),
            name: source.name(),
        }
    }
}
