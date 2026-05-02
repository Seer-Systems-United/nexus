//! # Emerson extraction utilities
//!
//! Helper functions for parsing Excel cells and normalizing text
//! when extracting data from Emerson workbooks.

pub(crate) const TOPLINE_SHEET_NAME: &str = "Topline Results";
pub(crate) const CROSSTABS_SHEET_NAME: &str = "crosstabs";
pub(crate) const FULL_CROSSTABS_SHEET_NAME: &str = "full crosstabs";
const SELECTED_CHOICE_SUFFIX: &str = " - Selected Choice";

/// Normalize whitespace in text: collapse runs of whitespace into single spaces.
pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Get the text content of a cell, or empty string if out of bounds.
///
/// # Parameters
/// - `rows`: All rows in the sheet.
/// - `row`: Row index.
/// - `col`: Column index.
pub(crate) fn cell_text(rows: &[Vec<calamine::Data>], row: usize, col: usize) -> String {
    rows.get(row)
        .and_then(|cells| cells.get(col))
        .map(cell_to_string)
        .unwrap_or_default()
}

/// Convert a calamine cell to a string, with empty for `Data::Empty`.
pub(crate) fn cell_to_string(cell: &calamine::Data) -> String {
    match cell {
        calamine::Data::Empty => String::new(),
        _ => normalize_text(&cell.to_string()),
    }
}

/// Get a numeric value from a cell, handling Float, Int, and string parsing.
///
/// # Returns
/// - `Some(f64)` if the cell contains a valid number.
/// - `None` if out of bounds or not numeric.
pub(crate) fn cell_number(rows: &[Vec<calamine::Data>], row: usize, col: usize) -> Option<f64> {
    let cell = rows.get(row)?.get(col)?;
    match cell {
        calamine::Data::Float(value) => Some(*value),
        calamine::Data::Int(value) => Some(*value as f64),
        _ => cell.to_string().trim().parse().ok(),
    }
}

/// Check if a row is entirely empty (all cells are empty strings).
pub(crate) fn row_is_empty(row: &[calamine::Data]) -> bool {
    row.iter().all(|cell| cell_to_string(cell).is_empty())
}

/// Check if a label is a "Total" row (case-insensitive).
pub(crate) fn is_total_label(label: &str) -> bool {
    label.eq_ignore_ascii_case("total")
}

/// Scale a percentage value: if <= 1.5, multiply by 100 (assume 0-1 scale).
pub(crate) fn scale_percent(value: f64) -> f32 {
    if value.abs() <= 1.5 {
        (value * 100.0) as f32
    } else {
        value as f32
    }
}

/// Clean a question title by removing the " - Selected Choice" suffix if present.
pub(crate) fn clean_question_title(title: &str) -> String {
    let normalized = normalize_text(title);
    normalized
        .strip_suffix(SELECTED_CHOICE_SUFFIX)
        .unwrap_or(&normalized)
        .trim()
        .to_string()
}
