use crate::api::error::ApiError;
use crate::sources::Scope;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopicQuery {
    pub scope: Option<String>,
    pub count: Option<u32>,
    pub n: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HeadlineQuery {
    pub scope: Option<String>,
    pub count: Option<u32>,
    pub n: Option<u32>,
    pub min_polls: Option<usize>,
}

pub fn parse_topic_scope(query: &TopicQuery) -> Result<Scope, ApiError> {
    parse_scope(
        query.scope.as_deref(),
        query.count.or(query.n),
        Scope::Latest,
    )
}

pub(super) fn parse_scope(
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
