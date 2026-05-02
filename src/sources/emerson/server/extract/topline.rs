//! # Emerson topline sheet parser
//!
//! Extracts bar graph data from Emerson "Topline Results" sheets.
//! Parses question headers and frequency/percent rows.

use super::utils::*;

/// Parse a topline sheet into a list of bar graph data structures.
///
/// Iterates through rows looking for question headers, then extracts
/// answer labels and valid percent values until an empty row is found.
///
/// # Parameters
/// - `rows`: All rows from the topline sheet.
///
/// # Returns
/// - `Vec<DataStructure>`: Bar graphs for each valid question found.
pub(crate) fn parse_topline_sheet(
    rows: &[Vec<calamine::Data>],
) -> Vec<crate::sources::DataStructure> {
    let mut charts = Vec::new();
    let mut index = 0usize;

    while index < rows.len() {
        let question = cell_text(rows, index, 0);
        if question.is_empty() {
            index += 1;
            continue;
        }

        // Find the header row (should have "Frequency" and "Valid Percent" columns)
        let mut header_row = index + 1;
        while header_row < rows.len() && row_is_empty(&rows[header_row]) {
            header_row += 1;
        }

        if header_row >= rows.len()
            || cell_text(rows, header_row, 2) != "Frequency"
            || cell_text(rows, header_row, 3) != "Valid Percent"
        {
            index += 1;
            continue;
        }

        let mut labels = Vec::new();
        let mut values = Vec::new();
        let mut cursor = header_row + 1;

        while cursor < rows.len() {
            if row_is_empty(&rows[cursor]) {
                if !labels.is_empty() {
                    break;
                }
                cursor += 1;
                continue;
            }

            // Non-empty first column indicates a new question section
            if !cell_text(rows, cursor, 0).is_empty() {
                break;
            }

            let label = cell_text(rows, cursor, 1);
            let percent = cell_number(rows, cursor, 3);

            if !label.is_empty() && !is_total_label(&label) {
                if let Some(percent) = percent {
                    labels.push(label);
                    values.push(percent as f32);
                }
            }

            cursor += 1;
        }

        if !labels.is_empty() {
            charts.push(crate::sources::DataStructure::BarGraph {
                title: question,
                x: labels,
                y: values,
                y_unit: "%".to_string(),
            });
            index = cursor;
            continue;
        }

        index += 1;
    }

    charts
}
