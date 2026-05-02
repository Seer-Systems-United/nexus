//! # Stable topic endpoint handlers
//!
//! Sub-module for stable canonical topic endpoints (presidential approval,
//! right direction, generic ballot, important problem).

pub mod core;
pub mod issues;

pub use core::{get_presidential_approval, get_right_direction};
pub use issues::{get_generic_ballot, get_important_problem};

use crate::api::error::ApiError;
use crate::api::topics::TopicQuery;
use crate::topics::types::TopicCollection;
use axum::Json;

/// Shared handler for stable topic endpoints that delegates to `topic_collection`.
///
/// # Parameters
/// - `topic_id`: Static topic identifier string.
/// - `query`: Topic query with scope parameters.
///
/// # Returns
/// - `Ok(Json<TopicCollection>)`: The topic data collection.
///
/// # Errors
/// - `400 Bad Request`: Invalid scope query.
/// - `404 Not Found`: Topic data not found.
/// - `503 Service Unavailable`: Topic data failed to load.
async fn stable_topic_endpoint(
    topic_id: &'static str,
    query: TopicQuery,
) -> Result<Json<TopicCollection>, ApiError> {
    super::topic_collection(topic_id, &query).await
}
