//! # CLI parsing utilities
//!
//! Helpers for parsing CLI argument values.

use crate::sources::Scope;
use crate::topics::enrichment::DynError;
use std::io::{Error as IoError, ErrorKind};

/// Get a required argument value by index.
pub(super) fn required_arg(args: &[String], index: usize, flag: &str) -> Result<String, DynError> {
    args.get(index).cloned().ok_or_else(|| {
        Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} requires a value"),
        )) as DynError
    })
}

/// Parse a u32 argument value.
pub(super) fn parse_u32(value: impl AsRef<str>, flag: &str) -> Result<u32, DynError> {
    let parsed = value.as_ref().parse::<u32>().map_err(|error| {
        IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} must be a positive integer: {error}"),
        )
    })?;

    if parsed == 0 {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} must be greater than zero"),
        )));
    }

    Ok(parsed)
}

/// Parse a usize argument value.
pub(super) fn parse_usize(value: impl AsRef<str>) -> Result<usize, DynError> {
    let parsed = value.as_ref().parse::<usize>().map_err(|error| {
        IoError::new(
            ErrorKind::InvalidInput,
            format!("--limit must be a positive integer: {error}"),
        )
    })?;

    if parsed == 0 {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            "--limit must be greater than zero",
        )));
    }

    Ok(parsed)
}

/// Parse the scope argument for the enrichment CLI.
///
/// # Parameters
/// - `scope`: Optional scope string.
/// - `count`: Optional count value.
pub fn parse_scope(scope: Option<&str>, count: Option<u32>) -> Result<Scope, DynError> {
    let normalized = scope
        .unwrap_or("last_entries")
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_");
    let count = count.unwrap_or(5);

    match normalized.as_str() {
        "" | "latest" => Ok(Scope::Latest),
        "last_n_entries" | "last_entries" | "entries" => Ok(Scope::LastNEntries(count)),
        "last_days" | "days" => Ok(Scope::LastDays(count)),
        "last_weeks" | "weeks" => Ok(Scope::LastWeeks(count)),
        "last_months" | "months" => Ok(Scope::LastMonths(count)),
        "last_years" | "years" => Ok(Scope::LastYears(count)),
        _ => Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("unsupported enrich-topics scope: {normalized}"),
        ))),
    }
}
