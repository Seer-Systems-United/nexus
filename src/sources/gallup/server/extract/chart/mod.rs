mod table;
mod text;
mod trend;

use crate::sources::DataStructure;
use text::normalize_line;

pub fn parse_chart_csv(title: &str, csv_bytes: &[u8]) -> Option<DataStructure> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(csv_bytes);
    let headers = reader
        .headers()
        .ok()?
        .iter()
        .map(normalize_line)
        .collect::<Vec<_>>();

    if headers.len() < 2 {
        return None;
    }

    let rows = reader
        .records()
        .flatten()
        .map(|record| record.iter().map(normalize_line).collect::<Vec<_>>())
        .filter(|cells| cells.iter().any(|cell| !cell.is_empty()))
        .collect::<Vec<_>>();

    if rows.is_empty() {
        return None;
    }

    trend::line_graph_from_rows(title, &headers, &rows)
        .or_else(|| table::crosstab_from_rows(title, &headers, rows))
}
