use crate::sources::{DataRow, emerson::server::extract::utils::*};

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
