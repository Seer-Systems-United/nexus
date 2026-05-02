//! # Gallup extraction module
//!
//! Extracts poll data from Gallup articles and charts.
//! Parses CSV data into bar graphs, line graphs, and crosstabs.

mod chart;
mod summary;

pub use chart::parse_chart_csv;

use crate::sources::{DataCollection, Scope};
use std::error::Error;

type DynError = Box<dyn Error + Send + Sync>;

/// Create a parse error with a static message.
fn parse_error(message: &'static str) -> DynError {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message).into()
}

/// Extract Gallup data from downloaded articles.
///
/// Parses chart CSVs into data structures, with fallback to unstructured text
/// if no charts are found.
///
/// # Parameters
/// - `articles`: Downloaded Gallup articles with charts.
/// - `scope`: The query scope.
///
/// # Returns
/// - `Ok(DataCollection)`: Combined data from all articles.
///
/// # Errors
/// - Returns an error if no valid data is found.
pub(crate) fn extract_gallup_data(
    articles: &[crate::sources::gallup::server::GallupArticleAsset],
    scope: Scope,
) -> Result<DataCollection, DynError> {
    let mut data = Vec::new();
    let mut chart_failures = 0usize;
    let mut skipped_articles = 0usize;
    let mut pdf_articles = 0usize;

    for article in articles {
        let before_count = data.len();
        if article.pdf_bytes.is_some() {
            pdf_articles += 1;
        }

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
        article_titles = ?articles.iter().map(|article| &article.title).collect::<Vec<_>>(),
        pdf_articles,
        chart_failures,
        skipped_articles,
        "extracted Gallup source data"
    );

    Ok(DataCollection {
        title: "Gallup Polls".to_string(),
        subtitle: summary::collection_subtitle(scope, articles),
        data,
    })
}
