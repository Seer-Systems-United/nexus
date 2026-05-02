use super::SourceQuery;
use crate::api::error::ApiError;
use crate::sources::Scope;

pub fn parse_scope(query: SourceQuery) -> Result<Scope, ApiError> {
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
