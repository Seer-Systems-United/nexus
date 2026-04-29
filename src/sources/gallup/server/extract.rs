use crate::sources::{DataCollection, DataPanel, DataRow, DataSeries, DataStructure, Scope};
use std::error::Error;

type DynError = Box<dyn Error + Send + Sync>;

fn parse_error(message: &'static str) -> DynError {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message).into()
}

fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_number(value: &str) -> Option<f32> {
    let normalized = value
        .trim()
        .trim_end_matches('%')
        .replace(',', "")
        .replace('<', "");

    if normalized.is_empty() || normalized == "-" || normalized.eq_ignore_ascii_case("n/a") {
        return None;
    }

    normalized.parse().ok()
}

fn looks_temporal(label: &str) -> bool {
    let normalized = label.trim();

    normalized.len() == 4 && normalized.chars().all(|ch| ch.is_ascii_digit())
        || normalized.contains('/')
        || normalized.contains('-')
}

fn parse_chart_csv(title: &str, csv_bytes: &[u8]) -> Option<DataStructure> {
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

    let mut rows = Vec::new();
    for record in reader.records().flatten() {
        let cells = record.iter().map(normalize_line).collect::<Vec<_>>();
        if cells.iter().all(|cell| cell.is_empty()) {
            continue;
        }
        rows.push(cells);
    }

    if rows.is_empty() {
        return None;
    }

    let x = rows
        .iter()
        .filter_map(|row| row.first())
        .filter(|label| !label.is_empty())
        .cloned()
        .collect::<Vec<_>>();

    let temporal_rows = x.iter().filter(|label| looks_temporal(label)).count();
    if temporal_rows > 0 && temporal_rows * 2 >= x.len() {
        let series = headers
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(index, header)| {
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
                    label: header.clone(),
                    values: parsed_values
                        .into_iter()
                        .map(Option::unwrap_or_default)
                        .collect(),
                })
            })
            .collect::<Vec<_>>();

        if !series.is_empty() {
            return Some(DataStructure::LineGraph {
                title: title.to_string(),
                x,
                series,
                y_unit: "%".to_string(),
            });
        }
    }

    let columns = headers.iter().skip(1).cloned().collect::<Vec<_>>();
    let data_rows = rows
        .into_iter()
        .filter_map(|row| {
            let label = row.first().cloned().unwrap_or_default();
            if label.is_empty() {
                return None;
            }

            let values = (1..headers.len())
                .map(|index| {
                    row.get(index)
                        .and_then(|cell| parse_number(cell))
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>();

            Some(DataRow { label, values })
        })
        .collect::<Vec<_>>();

    if columns.is_empty() || data_rows.is_empty() {
        return None;
    }

    Some(DataStructure::Crosstab {
        title: title.to_string(),
        prompt: title.to_string(),
        panels: vec![DataPanel {
            columns,
            groups: vec![crate::sources::DataGroup {
                title: headers[0].clone(),
                labels: headers.iter().skip(1).cloned().collect(),
            }],
            rows: data_rows,
        }],
        y_unit: "%".to_string(),
    })
}

pub fn extract_gallup_data(
    articles: &[crate::sources::gallup::server::GallupArticleAsset],
    scope: Scope,
) -> Result<DataCollection, DynError> {
    let mut data = Vec::new();
    let mut chart_failures = 0usize;
    let mut skipped_articles = 0usize;

    for article in articles {
        let before_count = data.len();

        for chart in &article.charts {
            match parse_chart_csv(&chart.title, &chart.csv_bytes) {
                Some(chart_data) => data.push(chart_data),
                None => chart_failures += 1,
            }
        }

        if data.len() == before_count {
            skipped_articles += 1;
        }
    }

    if data.is_empty() {
        return Err(parse_error("no Gallup source data found"));
    }

    tracing::debug!(
        source = "gallup",
        scope = %scope,
        charts = data.len(),
        chart_failures,
        skipped_articles,
        "extracted Gallup source data"
    );

    Ok(DataCollection {
        title: "Gallup Polls".to_string(),
        subtitle: collection_subtitle(scope, articles),
        data,
    })
}

fn collection_subtitle(
    scope: Scope,
    articles: &[crate::sources::gallup::server::GallupArticleAsset],
) -> Option<String> {
    let first = articles.first()?;
    let last = articles.last().unwrap_or(first);
    let article_label = if articles.len() == 1 {
        "article"
    } else {
        "articles"
    };

    Some(format!(
        "{} collection: {} to {} ({} {article_label})",
        scope.collection_label(),
        last.published_on,
        first.published_on,
        articles.len()
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_chart_csv;
    use crate::sources::DataStructure;

    #[test]
    fn parses_breakout_csv_as_crosstab() {
        let chart = parse_chart_csv(
            "Community involvement",
            b"Breakouts,Have volunteered,No but wanted to,No and not wanted to\nAll adults,39,24,37\nMore free time,41%,20%,39%\n",
        )
        .expect("chart should parse");

        let DataStructure::Crosstab { panels, .. } = chart else {
            panic!("expected crosstab");
        };

        assert_eq!(panels[0].columns[0], "Have volunteered");
        assert_eq!(panels[0].rows[0].label, "All adults");
        assert_eq!(panels[0].rows[0].values, vec![39.0, 24.0, 37.0]);
    }

    #[test]
    fn parses_temporal_csv_as_line_graph() {
        let chart = parse_chart_csv(
            "Approval trend",
            b"Year,Approve,Disapprove\n2024,45,50\n2025,47,49\n",
        )
        .expect("chart should parse");

        let DataStructure::LineGraph { x, series, .. } = chart else {
            panic!("expected line graph");
        };

        assert_eq!(x, vec!["2024", "2025"]);
        assert_eq!(series[0].label, "Approve");
        assert_eq!(series[0].values, vec![45.0, 47.0]);
    }
}
