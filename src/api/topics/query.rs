//! # Topic query parsing utilities
//!
//! Defines query structures and parsing logic for topic endpoint requests,
//! converting string scopes into typed `Scope` values.

use crate::api::error::ApiError;
use crate::sources::Scope;
use serde::Deserialize;

/// Query parameters for topic data requests.
///
/// # Fields
/// - `scope`: Optional scope string (e.g., "latest", "last_7_days").
/// - `count`: Optional count for scoped queries (alias: `n`).
/// - `n`: Alias for `count`.
#[derive(Debug, Deserialize)]
pub struct TopicQuery {
    pub scope: Option<String>,
    pub count: Option<u32>,
    pub n: Option<u32>,
}

/// Extended query parameters for headline topic requests.
///
/// # Fields
/// - `scope`: Optional scope string.
/// - `count`: Optional count for scoped queries (alias: `n`).
/// - `n`: Alias for `count`.
/// - `min_polls`: Minimum number of recent matching polls required.
#[derive(Debug, Deserialize)]
pub struct HeadlineQuery {
    pub scope: Option<String>,
    pub count: Option<u32>,
    pub n: Option<u32>,
    pub min_polls: Option<usize>,
}

/// Parse a `TopicQuery` into a typed `Scope` value.
///
/// # Parameters
/// - `query`: The topic query containing optional scope and count parameters.
///
/// # Returns
/// - `Ok(Scope)`: The parsed scope (defaults to `Scope::Latest` if not specified).
///
/// # Errors
/// - `400 Bad Request`: Unsupported scope string or missing count for counted scopes.
pub fn parse_topic_scope(query: &TopicQuery) -> Result<Scope, ApiError> {
    parse_scope(
        query.scope.as_deref(),
        query.count.or(query.n),
        Scope::Latest,
    )
}

/// Parse a scope string and count into a typed `Scope` value.
///
/// # Parameters
/// - `scope`: Optional scope string to parse.
/// - `count`: Optional count value for scoped queries.
/// - `default_scope`: Default scope to use if none is specified.
///
/// # Returns
/// - `Ok(Scope)`: The parsed scope value.
///
/// # Errors
/// - `400 Bad Request`: Unsupported scope string or missing/zero count.
pub(super) fn parse_scope(
    scope: Option<&str>,
    count: Option<u32>,
    default_scope: Scope,
) -> Result<Scope, ApiError> {
    let Some(scope) = scope else {
        return Ok(default_scope);
    };
    // Normalize scope: lowercase and replace hyphens with underscores
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

/// Validate and return the count parameter for scoped queries.
///
/// # Parameters
/// - `count`: Optional count value.
///
/// # Returns
/// - `Ok(u32)`: The validated count (must be non-zero).
///
/// # Errors
/// - `400 Bad Request`: Count is missing or zero.
fn required_count(count: Option<u32>) -> Result<u32, ApiError> {
    let count = count.ok_or_else(|| ApiError::bad_request("scope count is required"))?;

    if count == 0 {
        return Err(ApiError::bad_request(
            "scope count must be greater than zero",
        ));
    }

    Ok(count)
}
