use super::stable_topic_endpoint;
use crate::api::error::ApiError;
use crate::api::topics::TopicQuery;
use crate::topics::types::TopicCollection;
use axum::Json;
use axum::extract::Query;

#[utoipa::path(
    get,
    path = "/presidential-approval",
    tag = "Topics",
    params(
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years"),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Alias: n"),
        ("n" = Option<u32>, Query, description = "Alias for count"),
    ),
    responses(
        (status = 200, description = "Presidential approval topic data", body = TopicCollection),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Topic data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn get_presidential_approval(
    Query(query): Query<TopicQuery>,
) -> Result<Json<TopicCollection>, ApiError> {
    stable_topic_endpoint("presidential-approval", query).await
}

#[utoipa::path(
    get,
    path = "/right-direction",
    tag = "Topics",
    params(
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years"),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Alias: n"),
        ("n" = Option<u32>, Query, description = "Alias for count"),
    ),
    responses(
        (status = 200, description = "Right direction / wrong track topic data", body = TopicCollection),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Topic data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn get_right_direction(
    Query(query): Query<TopicQuery>,
) -> Result<Json<TopicCollection>, ApiError> {
    stable_topic_endpoint("right-direction", query).await
}
