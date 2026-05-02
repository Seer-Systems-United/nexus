use super::text::parse_number;
use crate::sources::{DataGroup, DataPanel, DataRow, DataStructure};

pub(super) fn crosstab_from_rows(
    title: &str,
    headers: &[String],
    rows: Vec<Vec<String>>,
) -> Option<DataStructure> {
    let columns = headers.iter().skip(1).cloned().collect::<Vec<_>>();
    let data_rows = rows
        .into_iter()
        .filter_map(|row| data_row_from_cells(&row, headers.len()))
        .collect::<Vec<_>>();

    if columns.is_empty() || data_rows.is_empty() {
        return None;
    }

    Some(DataStructure::Crosstab {
        title: title.to_string(),
        prompt: title.to_string(),
        panels: vec![DataPanel {
            columns,
            groups: vec![DataGroup {
                title: headers[0].clone(),
                labels: headers.iter().skip(1).cloned().collect(),
            }],
            rows: data_rows,
        }],
        y_unit: "%".to_string(),
    })
}

fn data_row_from_cells(row: &[String], headers_len: usize) -> Option<DataRow> {
    let label = row.first().cloned().unwrap_or_default();
    if label.is_empty() {
        return None;
    }

    let values = (1..headers_len)
        .map(|index| {
            row.get(index)
                .and_then(|cell| parse_number(cell))
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    Some(DataRow { label, values })
}
