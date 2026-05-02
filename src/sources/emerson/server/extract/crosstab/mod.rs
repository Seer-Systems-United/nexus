//! # Emerson crosstab sheet parser
//!
//! Extracts crosstab (cross-tabulation) data from Emerson workbooks.
//! Identifies question blocks and parses rows with percentage values.

mod layout;
mod row;

use super::utils::{cell_text, clean_question_title};
use layout::parse_crosstab_layout;
use row::parse_crosstab_row;

/// Parse a crosstab sheet into crosstab data structures.
///
/// First identifies the layout (columns, groups, row indices),
/// then iterates through question blocks to extract data rows.
///
/// # Parameters
/// - `rows`: All rows from the crosstab sheet.
///
/// # Returns
/// - `Vec<DataStructure>`: Crosstab structures for each question block.
pub fn parse_crosstab_sheet(rows: &[Vec<calamine::Data>]) -> Vec<crate::sources::DataStructure> {
    let Some(layout) = parse_crosstab_layout(rows) else {
        return Vec::new();
    };

    let max_cols = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut block_starts = Vec::new();

    // Find question block starts (non-empty cells in row 0)
    for col in 2..max_cols {
        let title = cell_text(rows, 0, col);
        if !title.is_empty() {
            block_starts.push((col, clean_question_title(&title)));
        }
    }

    let mut structures = Vec::new();

    for (index, (start_col, block_title)) in block_starts.iter().enumerate() {
        let end_col = block_starts
            .get(index + 1)
            .map(|(next_start, _)| next_start.saturating_sub(1))
            .unwrap_or(max_cols.saturating_sub(1));

        let mut data_rows = Vec::new();
        let mut option_col = *start_col;

        // Iterate through option/percent column pairs
        while option_col <= end_col {
            let answer_label = cell_text(rows, 1, option_col);
            if answer_label.is_empty() {
                option_col += 1;
                continue;
            }

            let percent_col = option_col + 1;
            if percent_col > end_col {
                break;
            }

            if let Some(row) =
                parse_crosstab_row(rows, answer_label, &layout.row_indices, percent_col)
            {
                data_rows.push(row);
            }

            option_col += 2;
        }

        if data_rows.is_empty() {
            continue;
        }

        structures.push(crate::sources::DataStructure::Crosstab {
            title: block_title.clone(),
            prompt: block_title.clone(),
            panels: vec![crate::sources::DataPanel {
                columns: layout.columns.clone(),
                groups: layout.groups.clone(),
                rows: data_rows,
            }],
            y_unit: "%".to_string(),
        });
    }

    structures
}
