//! # Gallup chart text utilities
//!
//! Helper functions for normalizing text and parsing numbers
//! from Gallup chart CSV data.

/// Normalize a line by collapsing whitespace.
pub(super) fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Parse a string into an f32 percentage value.
///
/// Handles '%' suffix, comma-separated numbers, and special values.
///
/// # Parameters
/// - `value`: The string to parse.
///
/// # Returns
/// - `Some(f32)` if parsing succeeds.
/// - `None` for empty, "-", "N/A", or invalid values.
pub(super) fn parse_number(value: &str) -> Option<f32> {
    let normalized = value
        .trim()
        .trim_end_matches('%')
        .replace(',', "")
        .replace('<', "");

    if normalized.is_empty() || normalized == "-" || normalized.eq_ignore_ascii_case("n/a") {
        return None;
    }

    normalized.parse().ok()
}

/// Check if a label looks like a temporal value (year, date, etc.).
///
/// # Parameters
/// - `label`: The label to check.
///
/// # Returns
/// - `true` if the label is 4 digits, contains '/' or '-'.
pub(super) fn looks_temporal(label: &str) -> bool {
    let normalized = label.trim();

    normalized.len() == 4 && normalized.chars().all(|ch| ch.is_ascii_digit())
        || normalized.contains('/')
        || normalized.contains('-')
}
