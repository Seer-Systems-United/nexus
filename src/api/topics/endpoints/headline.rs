//! # Headline topic endpoints
//!
//! Handles listing stable topics and discovering recurring headline topics
//! from recent polling data.

use crate::api::error::ApiError;
use crate::api::topics::{HeadlineQuery, query};
use crate::sources::Scope;
use crate::topics::types::{HeadlineTopicSummary, TopicSummary};
use axum::Json;
use axum::extract::Query;

#[utoipa::path(
    get,
    path = "/",
    tag = "Topics",
    responses(
        (status = 200, description = "Stable canonical polling topics", body = [TopicSummary]),
    )
)]
/// Handle GET /topics/ to list all stable canonical topics.
///
/// # Returns
/// - `Json<Vec<TopicSummary>>`: List of stable topic summaries.
pub async fn list_topics() -> Json<Vec<TopicSummary>> {
    Json(crate::topics::catalog::stable_topics())
}

#[utoipa::path(
    get,
    path = "/headlines",
    tag = "Topics",
    params(
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years. Defaults to last_n_entries."),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Defaults to 5 for headlines."),
        ("n" = Option<u32>, Query, description = "Alias for count"),
        ("min_polls" = Option<usize>, Query, description = "Minimum number of recent matching poll questions required. Defaults to 2."),
    ),
    responses(
        (status = 200, description = "Recent non-stable topics that recur across source polls", body = [HeadlineTopicSummary]),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Topic data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
/// Handle GET /topics/headlines to discover recurring headline topics.
///
/// # Parameters
/// - `query`: Query parameters for scope, count, and minimum poll threshold.
///
/// # Returns
/// - `Ok(Json<Vec<HeadlineTopicSummary>>)`: List of recurring headline topics.
///
/// # Errors
/// - `400 Bad Request`: Invalid scope query.
/// - `503 Service Unavailable`: Topic data failed to load.
pub async fn get_headline_topics(
    Query(query): Query<HeadlineQuery>,
) -> Result<Json<Vec<HeadlineTopicSummary>>, ApiError> {
    let min_polls = query.min_polls.unwrap_or(2).max(1);
    let scope = query::parse_scope(
        query.scope.as_deref(),
        query.count.or(query.n).or(Some(5)),
        Scope::LastNEntries(5),
    )?;

    crate::topics::service::headline_topics(scope, min_polls)
        .await
        .map(Json)
        .map_err(super::topic_error)
}
