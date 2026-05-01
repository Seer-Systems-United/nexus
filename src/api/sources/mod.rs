use axum::Json;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::error::ApiError;
use crate::sources::{DataCollection, Scope, Source};

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

#[derive(Debug, Serialize, ToSchema)]
pub struct SourceSummary {
    pub id: &'static str,
    pub name: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct SourceQuery {
    scope: Option<String>,
    count: Option<u32>,
    n: Option<u32>,
    pub question: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum SourceKey {
    Emerson,
    Gallup,
    Ipsos,
    YouGov,
}

impl SourceKey {
    const ALL: [Self; 4] = [Self::Emerson, Self::Gallup, Self::Ipsos, Self::YouGov];

    fn parse(input: &str) -> Option<Self> {
        match input.trim().to_ascii_lowercase().as_str() {
            "emerson" => Some(Self::Emerson),
            "gallup" => Some(Self::Gallup),
            "ipsos" => Some(Self::Ipsos),
            "yougov" | "you-gov" => Some(Self::YouGov),
            _ => None,
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::Emerson => "emerson",
            Self::Gallup => "gallup",
            Self::Ipsos => "ipsos",
            Self::YouGov => "yougov",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Emerson => "Emerson",
            Self::Gallup => "Gallup",
            Self::Ipsos => "Ipsos",
            Self::YouGov => "YouGov",
        }
    }

    fn summary(self) -> SourceSummary {
        SourceSummary {
            id: self.id(),
            name: self.name(),
        }
    }
}

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
    Json(SourceKey::ALL.into_iter().map(SourceKey::summary).collect())
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
    let source =
        SourceKey::parse(&source).ok_or_else(|| ApiError::not_found("source not found"))?;
    let question_filter = query.question.clone();
    let scope = parse_scope(query)?;
    let mut data = load_source(source, scope).await?;

    if let Some(question) = question_filter {
        let filter_lower = question.to_lowercase();
        data.data.retain(|structure| match structure {
            crate::sources::DataStructure::BarGraph { title, .. } => {
                title.to_lowercase().contains(&filter_lower)
            }
            crate::sources::DataStructure::LineGraph { title, .. } => {
                title.to_lowercase().contains(&filter_lower)
            }
            crate::sources::DataStructure::PieChart { title, .. } => {
                title.to_lowercase().contains(&filter_lower)
            }
            crate::sources::DataStructure::Crosstab { title, prompt, .. } => {
                title.to_lowercase().contains(&filter_lower)
                    || prompt.to_lowercase().contains(&filter_lower)
            }
            crate::sources::DataStructure::Unstructured { data } => {
                data.to_lowercase().contains(&filter_lower)
            }
        });
    }

    Ok(Json(data))
}

#[tracing::instrument(name = "source.load", skip_all, fields(source = source.id(), scope = %scope))]
async fn load_source(source: SourceKey, scope: Scope) -> Result<DataCollection, ApiError> {
    tracing::info!("loading source data");

    let data = match source {
        SourceKey::Emerson => crate::sources::emerson::Emerson::get_data(scope),
        SourceKey::Gallup => crate::sources::gallup::Gallup::get_data(scope),
        SourceKey::Ipsos => crate::sources::ipsos::Ipsos::get_data(scope),
        SourceKey::YouGov => crate::sources::yougov::YouGov::get_data(scope),
    }
    .await
    .map_err(|error| {
        tracing::error!(error = %error, "failed to load source data");
        ApiError::service_unavailable("source data unavailable")
    })?;

    tracing::info!(
        structures = data.data.len(),
        title = %data.title,
        "loaded source data"
    );

    Ok(data)
}

fn parse_scope(query: SourceQuery) -> Result<Scope, ApiError> {
    let Some(scope) = query.scope.as_deref() else {
        return Ok(Scope::Latest);
    };
    let normalized = scope.trim().to_ascii_lowercase().replace('-', "_");

    match normalized.as_str() {
        "" | "latest" => Ok(Scope::Latest),
        "last_n_entries" | "last_entries" | "entries" => {
            Ok(Scope::LastNEntries(required_count(&query)?))
        }
        "last_days" | "days" => Ok(Scope::LastDays(required_count(&query)?)),
        "last_weeks" | "weeks" => Ok(Scope::LastWeeks(required_count(&query)?)),
        "last_months" | "months" => Ok(Scope::LastMonths(required_count(&query)?)),
        "last_years" | "years" => Ok(Scope::LastYears(required_count(&query)?)),
        _ => Err(ApiError::bad_request(format!(
            "unsupported source scope: {scope}"
        ))),
    }
}

fn required_count(query: &SourceQuery) -> Result<u32, ApiError> {
    let count = query
        .count
        .or(query.n)
        .ok_or_else(|| ApiError::bad_request("scope count is required"))?;

    if count == 0 {
        return Err(ApiError::bad_request(
            "scope count must be greater than zero",
        ));
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::{SourceQuery, parse_scope};
    use crate::sources::Scope;

    #[test]
    fn defaults_to_latest_scope() {
        let scope = parse_scope(SourceQuery {
            scope: None,
            count: None,
            n: None,
            question: None,
        })
        .unwrap();

        assert_eq!(scope, Scope::Latest);
    }

    #[test]
    fn parses_counted_scopes() {
        let scope = parse_scope(SourceQuery {
            scope: Some("last_days".to_string()),
            count: Some(30),
            n: None,
            question: None,
        })
        .unwrap();

        assert_eq!(scope, Scope::LastDays(30));
    }

    #[test]
    fn rejects_missing_scope_count() {
        let error = parse_scope(SourceQuery {
            scope: Some("last_entries".to_string()),
            count: None,
            n: None,
            question: None,
        });

        assert!(error.is_err());
    }
}
