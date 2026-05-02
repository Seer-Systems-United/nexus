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
