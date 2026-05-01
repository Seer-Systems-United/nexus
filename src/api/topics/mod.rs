use axum::Json;
use axum::extract::{Path, Query};
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::error::ApiError;
use crate::sources::Scope;
use crate::topics::types::{HeadlineTopicSummary, TopicCollection, TopicSummary};

#[derive(OpenApi)]
#[openapi(
    paths(
        list_topics,
        get_topic,
        get_headline_topics,
        get_presidential_approval,
        get_right_direction,
        get_generic_ballot,
        get_important_problem
    ),
    components(schemas(
        crate::topics::types::TopicSummary,
        crate::topics::types::TopicCollection,
        crate::topics::types::TopicObservation,
        crate::topics::types::TopicSource,
        crate::topics::types::TopicStatus,
        crate::topics::types::Compatibility,
        crate::topics::types::DemographicGroup,
        crate::topics::types::DemographicValue,
        crate::topics::types::DemographicResult,
        crate::topics::types::AnswerResult,
        crate::topics::types::PooledAnswerResult,
        crate::topics::types::PooledDemographicResult,
        crate::topics::types::HeadlineTopicSummary,
        crate::sources::Scope,
        crate::api::error::ApiErrorBody,
    )),
    tags((name = "Topics", description = "Canonical polling question topics"))
)]
struct TopicsDoc;

#[derive(Debug, Deserialize)]
pub struct TopicQuery {
    scope: Option<String>,
    count: Option<u32>,
    n: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HeadlineQuery {
    scope: Option<String>,
    count: Option<u32>,
    n: Option<u32>,
    min_polls: Option<usize>,
}

pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(TopicsDoc::openapi())
        .routes(routes!(list_topics))
        .routes(routes!(get_headline_topics))
        .routes(routes!(get_presidential_approval))
        .routes(routes!(get_right_direction))
        .routes(routes!(get_generic_ballot))
        .routes(routes!(get_important_problem))
        .routes(routes!(get_topic))
}

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
    let scope = parse_scope(
        query.scope.as_deref(),
        query.count.or(query.n).or(Some(5)),
        Scope::LastNEntries(5),
    )?;

    crate::topics::service::headline_topics(scope, min_polls)
        .await
        .map(Json)
        .map_err(topic_error)
}

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
pub async fn get_important_problem(
    Query(query): Query<TopicQuery>,
) -> Result<Json<TopicCollection>, ApiError> {
    stable_topic_endpoint("important-problem", query).await
}

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
    let scope = parse_topic_scope(&query)?;

    crate::topics::service::get_topic(scope, &topic_id)
        .await
        .map(Json)
        .map_err(topic_error)
}

async fn stable_topic_endpoint(
    topic_id: &'static str,
    query: TopicQuery,
) -> Result<Json<TopicCollection>, ApiError> {
    let scope = parse_topic_scope(&query)?;

    crate::topics::service::get_topic(scope, topic_id)
        .await
        .map(Json)
        .map_err(topic_error)
}

fn parse_topic_scope(query: &TopicQuery) -> Result<Scope, ApiError> {
    parse_scope(
        query.scope.as_deref(),
        query.count.or(query.n),
        Scope::Latest,
    )
}

fn parse_scope(
    scope: Option<&str>,
    count: Option<u32>,
    default_scope: Scope,
) -> Result<Scope, ApiError> {
    let Some(scope) = scope else {
        return Ok(default_scope);
    };
    let normalized = scope.trim().to_ascii_lowercase().replace('-', "_");

    match normalized.as_str() {
        "" | "latest" => Ok(Scope::Latest),
        "last_n_entries" | "last_entries" | "entries" => {
            Ok(Scope::LastNEntries(required_count(count)?))
        }
        "last_days" | "days" => Ok(Scope::LastDays(required_count(count)?)),
        "last_weeks" | "weeks" => Ok(Scope::LastWeeks(required_count(count)?)),
        "last_months" | "months" => Ok(Scope::LastMonths(required_count(count)?)),
        "last_years" | "years" => Ok(Scope::LastYears(required_count(count)?)),
        _ => Err(ApiError::bad_request(format!(
            "unsupported topic scope: {scope}"
        ))),
    }
}

fn required_count(count: Option<u32>) -> Result<u32, ApiError> {
    let count = count.ok_or_else(|| ApiError::bad_request("scope count is required"))?;

    if count == 0 {
        return Err(ApiError::bad_request(
            "scope count must be greater than zero",
        ));
    }

    Ok(count)
}

fn topic_error(error: Box<dyn std::error::Error + Send + Sync>) -> ApiError {
    if let Some(io_error) = error.downcast_ref::<std::io::Error>()
        && io_error.kind() == std::io::ErrorKind::NotFound
    {
        return ApiError::not_found(io_error.to_string());
    }

    tracing::error!(error = %error, "failed to load topic data");
    ApiError::service_unavailable("topic data unavailable")
}

#[cfg(test)]
mod tests {
    use super::{TopicQuery, parse_topic_scope};
    use crate::sources::Scope;

    #[test]
    fn defaults_to_latest_scope() {
        let scope = parse_topic_scope(&TopicQuery {
            scope: None,
            count: None,
            n: None,
        })
        .unwrap();

        assert_eq!(scope, Scope::Latest);
    }

    #[test]
    fn parses_counted_scopes() {
        let scope = parse_topic_scope(&TopicQuery {
            scope: Some("last_entries".to_string()),
            count: Some(3),
            n: None,
        })
        .unwrap();

        assert_eq!(scope, Scope::LastNEntries(3));
    }
}
