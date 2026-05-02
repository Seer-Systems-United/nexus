//! # Issues stable topic endpoints
//!
//! Handles endpoints for generic ballot and important problem topics.

use super::stable_topic_endpoint;
use crate::api::error::ApiError;
use crate::api::topics::TopicQuery;
use crate::topics::types::TopicCollection;
use axum::Json;
use axum::extract::Query;

#[utoipa::path(
    get,
    path = "/generic-ballot",
    tag = "Topics",
    params(
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years"),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Alias: n"),
        ("n" = Option<u32>, Query, description = "Alias for count"),
    ),
    responses(
        (status = 200, description = "Generic congressional ballot topic data", body = TopicCollection),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Topic data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
/// Handle GET /topics/generic-ballot endpoint.
///
/// # Parameters
/// - `query`: Topic query with scope and count parameters.
///
/// # Returns
/// - `Ok(Json<TopicCollection>)`: Generic congressional ballot topic data.
///
/// # Errors
/// - `400 Bad Request`: Invalid scope query.
/// - `503 Service Unavailable`: Topic data failed to load.
pub async fn get_generic_ballot(
    Query(query): Query<TopicQuery>,
) -> Result<Json<TopicCollection>, ApiError> {
    stable_topic_endpoint("generic-ballot", query).await
}

#[utoipa::path(
    get,
    path = "/important-problem",
    tag = "Topics",
    params(
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years"),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Alias: n"),
        ("n" = Option<u32>, Query, description = "Alias for count"),
    ),
    responses(
        (status = 200, description = "Most important problem topic data", body = TopicCollection),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Topic data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
/// Handle GET /topics/important-problem endpoint.
///
/// # Parameters
/// - `query`: Topic query with scope and count parameters.
///
/// # Returns
/// - `Ok(Json<TopicCollection>)`: Most important problem topic data.
///
/// # Errors
/// - `400 Bad Request`: Invalid scope query.
/// - `503 Service Unavailable`: Topic data failed to load.
pub async fn get_important_problem(
    Query(query): Query<TopicQuery>,
) -> Result<Json<TopicCollection>, ApiError> {
    stable_topic_endpoint("important-problem", query).await
}
