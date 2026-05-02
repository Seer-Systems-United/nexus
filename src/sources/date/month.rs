//! # Month name parsing utilities
//!
//! Provides a function to parse month names (full or abbreviated)
//! into their numeric representation (1-12).

/// Parse a month name into its numeric value (1-12).
///
/// Accepts full month names (case-insensitive) or 3-letter abbreviations.
/// Extra whitespace and non-alphanumeric characters are trimmed.
///
/// # Parameters
/// - `input`: The month name or abbreviation to parse.
///
/// # Returns
/// - `Some(u8)`: The month number (1-12) if recognized.
/// - `None`: If the input doesn't match any known month.
pub fn parse_month_name(input: &str) -> Option<u8> {
    let normalized = input
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '+')
        .to_ascii_lowercase();

    match normalized.as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "sept" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}
