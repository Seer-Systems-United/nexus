//! # Emerson crosstab row parser
//!
//! Extracts a single data row from a crosstab sheet
//! for a given answer label and column indices.

use crate::sources::{DataRow, emerson::server::extract::utils::*};

/// Parse a crosstab data row for a specific answer label.
///
/// Iterates through the layout row indices, extracting percentage values
/// from the given percent column and scaling them.
///
/// # Parameters
/// - `rows`: All rows from the crosstab sheet.
/// - `label`: The answer label for this row.
/// - `row_indices`: Row indices that contain data values.
/// - `percent_col`: The column index for percentage values.
///
/// # Returns
/// - `Some(DataRow)` if valid data was found.
/// - `None` if label is empty or a "Total" row.
pub(super) fn parse_crosstab_row(
    rows: &[Vec<calamine::Data>],
    label: String,
    row_indices: &[usize],
    percent_col: usize,
) -> Option<DataRow> {
    if label.is_empty() || is_total_label(&label) {
        return None;
    }

    let values = row_indices
        .iter()
        .map(|row_index| {
            cell_number(rows, *row_index, percent_col)
                .map(scale_percent)
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    Some(DataRow { label, values })
}
