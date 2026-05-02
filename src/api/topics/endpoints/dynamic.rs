use crate::api::error::ApiError;
use crate::api::topics::TopicQuery;
use crate::topics::types::TopicCollection;
use axum::Json;
use axum::extract::{Path, Query};

#[utoipa::path(
    get,
    path = "/{topic_id}",
    tag = "Topics",
    params(
        ("topic_id" = String, Path, description = "Stable or headline topic id"),
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years"),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Alias: n"),
        ("n" = Option<u32>, Query, description = "Alias for count"),
    ),
    responses(
        (status = 200, description = "Canonical topic data by id", body = TopicCollection),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 404, description = "Unknown topic", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Topic data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn get_topic(
    Path(topic_id): Path<String>,
    Query(query): Query<TopicQuery>,
) -> Result<Json<TopicCollection>, ApiError> {
    super::topic_collection(&topic_id, &query).await
}
