//! # API error response body
//!
//! Defines the JSON body structure returned for API error responses.

use serde::Serialize;
use utoipa::ToSchema;

/// JSON body for API error responses.
///
/// # Fields
/// - `error`: Machine-readable error code string.
/// - `message`: Human-readable error description.
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub error: String,
    pub message: String,
}
