use super::text::{looks_temporal, parse_number};
use crate::sources::{DataSeries, DataStructure};

pub(super) fn line_graph_from_rows(
    title: &str,
    headers: &[String],
    rows: &[Vec<String>],
) -> Option<DataStructure> {
    let x = rows
        .iter()
        .filter_map(|row| row.first())
        .filter(|label| !label.is_empty())
        .cloned()
        .collect::<Vec<_>>();
    let temporal_rows = x.iter().filter(|label| looks_temporal(label)).count();

    if temporal_rows == 0 || temporal_rows * 2 < x.len() {
        return None;
    }

    let series = headers
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(index, header)| series_from_column(rows, index, header))
        .collect::<Vec<_>>();

    (!series.is_empty()).then(|| DataStructure::LineGraph {
        title: title.to_string(),
        x,
        series,
        y_unit: "%".to_string(),
    })
}

fn series_from_column(rows: &[Vec<String>], index: usize, header: &str) -> Option<DataSeries> {
    if header.is_empty() {
        return None;
    }

    let parsed_values = rows
        .iter()
        .map(|row| row.get(index).and_then(|cell| parse_number(cell)))
        .collect::<Vec<_>>();

    if parsed_values.iter().all(Option::is_none) {
        return None;
    }

    Some(DataSeries {
        label: header.to_string(),
        values: parsed_values
            .into_iter()
            .map(Option::unwrap_or_default)
            .collect(),
    })
}
