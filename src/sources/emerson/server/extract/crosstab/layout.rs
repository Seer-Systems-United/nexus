//! # Emerson crosstab layout parser
//!
//! Identifies the layout of a crosstab sheet: column headers,
//! group labels, and data row indices.

use crate::sources::{DataGroup, emerson::server::extract::utils::cell_text};

/// The layout of a crosstab sheet: columns, groups, and data row indices.
pub(super) struct CrosstabLayout {
    pub(super) columns: Vec<String>,
    pub(super) groups: Vec<DataGroup>,
    pub(super) row_indices: Vec<usize>,
}

/// Parse the layout of a crosstab sheet.
///
/// Scans rows starting from row 3 to identify group titles, column labels,
/// and data row indices. Stops when 3 consecutive empty rows are found.
///
/// # Parameters
/// - `rows`: All rows from the crosstab sheet.
///
/// # Returns
/// - `Some(CrosstabLayout)` if a valid layout was found.
/// - `None` if no columns were identified.
pub(super) fn parse_crosstab_layout(rows: &[Vec<calamine::Data>]) -> Option<CrosstabLayout> {
    let mut layout = LayoutBuilder::default();

    for row_index in 3..rows.len() {
        if !layout.add_row(rows, row_index) {
            break;
        }
    }

    (!layout.columns.is_empty()).then(|| layout.finish())
}

#[derive(Default)]
/// Builder for constructing a `CrosstabLayout` incrementally.
struct LayoutBuilder {
    columns: Vec<String>,
    groups: Vec<DataGroup>,
    row_indices: Vec<usize>,
    current_group_index: Option<usize>,
    empty_streak: usize,
}

impl LayoutBuilder {
    /// Add a row to the layout. Returns false if parsing should stop.
    fn add_row(&mut self, rows: &[Vec<calamine::Data>], row_index: usize) -> bool {
        let group_title = cell_text(rows, row_index, 0);
        let subgroup_label = cell_text(rows, row_index, 1);

        if group_title.is_empty() && subgroup_label.is_empty() {
            return self.record_empty_row();
        }

        self.empty_streak = 0;
        if !group_title.is_empty() {
            self.push_group(group_title);
        }
        if !subgroup_label.is_empty() {
            self.push_column(subgroup_label, row_index);
        }
        true
    }

    /// Record an empty row; stop after 3 consecutive empty rows.
    fn record_empty_row(&mut self) -> bool {
        if !self.columns.is_empty() {
            self.empty_streak += 1;
        }
        self.empty_streak < 3
    }

    /// Push a new group with the given title.
    fn push_group(&mut self, title: String) {
        self.groups.push(DataGroup {
            title,
            labels: Vec::new(),
        });
        self.current_group_index = Some(self.groups.len() - 1);
    }

    /// Push a column label and record its row index for data extraction.
    fn push_column(&mut self, label: String, row_index: usize) {
        let group_index = self.current_group_index.unwrap_or_else(|| {
            self.push_group("Overall".to_string());
            self.groups.len() - 1
        });
        self.groups[group_index].labels.push(label.clone());
        self.columns.push(label);
        self.row_indices.push(row_index);
    }

    /// Finish building and return the final `CrosstabLayout`.
    fn finish(self) -> CrosstabLayout {
        CrosstabLayout {
            columns: self.columns,
            groups: self.groups,
            row_indices: self.row_indices,
        }
    }
}
