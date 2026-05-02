//! # Source data loading utilities
//!
//! Provides helper functions for loading and filtering polling source data
//! with tracing instrumentation for observability.

use crate::api::error::ApiError;
use crate::sources::{DataCollection, DataStructure, Scope, SourceId};

#[tracing::instrument(name = "source.load", skip_all, fields(source = source.id(), scope = %scope))]
/// Load polling data from a given source and scope.
///
/// # Parameters
/// - `source`: The polling source to load data from.
/// - `scope`: The scope defining how much data to load (latest, last N days, etc.).
///
/// # Returns
/// - `Ok(DataCollection)`: The loaded source data with structures and metadata.
///
/// # Errors
/// - `503 Service Unavailable`: Source data failed to load.
pub(super) async fn load_source(
    source: SourceId,
    scope: Scope,
) -> Result<DataCollection, ApiError> {
    tracing::info!("loading source data");

    let data = source.load(scope).await.map_err(|error| {
        tracing::error!(error = %error, "failed to load source data");
        ApiError::service_unavailable("source data unavailable")
    })?;

    tracing::info!(
        structures = data.data.len(),
        title = %data.title,
        "loaded source data"
    );

    Ok(data)
}

/// Filter data structures to only those matching a question string.
///
/// # Parameters
/// - `data`: Mutable reference to the data collection to filter in-place.
/// - `question`: The question string to match against (case-insensitive).
pub(super) fn retain_question_matches(data: &mut DataCollection, question: &str) {
    let filter_lower = question.to_lowercase();
    data.data
        .retain(|structure| structure_text(structure).contains(&filter_lower));
}

/// Extract searchable text from a data structure for question matching.
fn structure_text(structure: &DataStructure) -> String {
    match structure {
        DataStructure::BarGraph { title, .. }
        | DataStructure::LineGraph { title, .. }
        | DataStructure::PieChart { title, .. } => title.to_lowercase(),
        DataStructure::Crosstab { title, prompt, .. } => {
            format!("{} {}", title.to_lowercase(), prompt.to_lowercase())
        }
        DataStructure::Unstructured { data } => data.to_lowercase(),
    }
}
