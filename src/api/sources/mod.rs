mod data;
mod models;
mod scope;

use axum::Json;
use axum::extract::{Path, Query};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::error::ApiError;
use crate::sources::{DataCollection, SourceId};
pub use models::{SourceQuery, SourceSummary};
pub use scope::parse_scope;

#[derive(OpenApi)]
#[openapi(
    paths(list_sources, get_source),
    components(schemas(
        SourceSummary,
        crate::sources::DataCollection,
        crate::sources::DataStructure,
        crate::sources::DataPanel,
        crate::sources::DataGroup,
        crate::sources::DataSeries,
        crate::sources::DataSlice,
        crate::sources::DataRow,
        crate::sources::Scope,
        crate::api::error::ApiErrorBody,
    )),
    tags((name = "Sources", description = "Polling source ingestion"))
)]
struct SourcesDoc;

pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(SourcesDoc::openapi())
        .routes(routes!(list_sources))
        .routes(routes!(get_source))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "Sources",
    responses(
        (status = 200, description = "Available polling sources", body = [SourceSummary]),
    )
)]
pub async fn list_sources() -> Json<Vec<SourceSummary>> {
    Json(SourceId::ALL.into_iter().map(SourceSummary::from).collect())
}

#[utoipa::path(
    get,
    path = "/{source}",
    tag = "Sources",
    params(
        ("source" = String, Path, description = "Source id: emerson, gallup, ipsos, or yougov"),
        ("scope" = Option<String>, Query, description = "Scope: latest, last_n_entries, last_days, last_weeks, last_months, or last_years"),
        ("count" = Option<u32>, Query, description = "Required for counted scopes. Alias: n"),
        ("n" = Option<u32>, Query, description = "Alias for count"),
        ("question" = Option<String>, Query, description = "Filter by question name or title"),
    ),
    responses(
        (status = 200, description = "Scoped source data", body = DataCollection),
        (status = 400, description = "Invalid scope query", body = crate::api::error::ApiErrorBody),
        (status = 404, description = "Unknown source", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Source data unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn get_source(
    Path(source): Path<String>,
    Query(query): Query<SourceQuery>,
) -> Result<Json<DataCollection>, ApiError> {
    let source = SourceId::parse(&source).ok_or_else(|| ApiError::not_found("source not found"))?;
    let question_filter = query.question.clone();
    let scope = parse_scope(query)?;
    let mut data = data::load_source(source, scope).await?;

    if let Some(question) = question_filter {
        data::retain_question_matches(&mut data, &question);
    }

    Ok(Json(data))
}
