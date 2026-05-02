//! # YouGov text parsing utilities
//!
//! Helper functions for normalizing text, detecting page numbers,
//! question titles, and parsing percentage values.

pub(crate) const DOCUMENT_TITLE: &str = "The Economist/YouGov Poll";

/// Normalize whitespace in a line: collapse runs into single spaces.
pub(crate) fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Check if a line is a page number (all digits).
pub(crate) fn is_page_number(line: &str) -> bool {
    !line.is_empty() && line.chars().all(|ch| ch.is_ascii_digit())
}

/// Check if a line looks like a question title.
///
/// Must start with a digit, followed by ". " or similar,
/// and contain an alphabetic character after.
pub(crate) fn is_question_title(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    if !matches!(chars.peek(), Some(ch) if ch.is_ascii_digit()) {
        return false;
    }
    while matches!(chars.peek(), Some(ch) if ch.is_ascii_digit()) {
        chars.next();
    }
    if matches!(chars.peek(), Some(ch) if ch.is_ascii_uppercase()) {
        chars.next();
    }
    matches!(chars.next(), Some('.')) && matches!(chars.next(), Some(' '))
}

/// Check if a line is a table of contents entry (contains ". " pattern).
pub(crate) fn is_table_of_contents_entry(line: &str) -> bool {
    line.contains(". ")
}

/// Parse a percentage token into an f32 value.
///
/// Strips trailing '%' and comma separators.
pub(crate) fn parse_percentage(token: &str) -> Option<f32> {
    token.strip_suffix('%')?.replace(',', "").parse().ok()
}

/// Parse a data row from a line with expected number of values.
///
/// # Parameters
/// - `line`: The line to parse.
/// - `expected_values`: Number of percentage values expected.
///
/// # Returns
/// - `Some(DataRow)` if valid data is found.
pub(crate) fn parse_row(line: &str, expected_values: usize) -> Option<crate::sources::DataRow> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }
    let mut value_count = 0usize;
    for token in tokens.iter().rev() {
        if parse_percentage(token).is_some() {
            value_count += 1;
        } else {
            break;
        }
    }
    if value_count != expected_values || value_count >= tokens.len() {
        return None;
    }
    let split_at = tokens.len() - value_count;
    let label = tokens[..split_at].join(" ");
    let values = tokens[split_at..]
        .iter()
        .filter_map(|token| parse_percentage(token))
        .collect::<Vec<_>>();

    Some(crate::sources::DataRow { label, values })
}
