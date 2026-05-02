//! # Demographic mapping from data structures
//!
//! Extracts demographic results from polling data structures
//! and normalizes them to canonical answer formats.

use crate::sources::{DataPanel, DataStructure};
use crate::topics::answers;
use crate::topics::demographics;
use crate::topics::types::{AnswerResult, DemographicResult};
use std::collections::HashSet;

/// Extract demographic results from a data structure.
///
/// # Parameters
/// - `topic_id`: The canonical topic ID.
/// - `structure`: The data structure to extract from.
///
/// # Returns
/// - `Vec<DemographicResult>`: Extracted demographic breakdowns.
pub fn demographics_from_structure(
    topic_id: &str,
    structure: &DataStructure,
) -> Vec<DemographicResult> {
    let demographics = match structure {
        DataStructure::Crosstab { panels, .. } => panels
            .iter()
            .flat_map(|panel| panel_demographics(topic_id, panel))
            .collect(),
        DataStructure::BarGraph { x, y, .. } => {
            let answers = answers::normalize_answers(
                topic_id,
                x.iter().zip(y.iter()).map(|(label, value)| (label.as_str(), *value)),
            );
            non_empty_total(answers).into_iter().collect()
        }
        DataStructure::PieChart { slices, .. } => {
            let answers = answers::normalize_answers(
                topic_id,
                slices
                    .iter()
                    .map(|slice| (slice.label.as_str(), slice.value)),
            );
            non_empty_total(answers).into_iter().collect()
        }
        DataStructure::LineGraph { series, .. } => {
            let answers = answers::normalize_answers(
                topic_id,
                series.iter().filter_map(|series| {
                    series
                        .values
                        .last()
                        .map(|value| (series.label.as_str(), *value))
                }),
            );
            non_empty_total(answers).into_iter().collect()
        }
        DataStructure::Unstructured { .. } => Vec::new(),
    };

    dedupe_demographics(demographics)
}

/// Extract demographic results from a single crosstab panel.
fn panel_demographics(topic_id: &str, panel: &DataPanel) -> Vec<DemographicResult> {
    panel
        .columns
        .iter()
        .enumerate()
        .filter_map(|(column_index, _)| {
            let answers = answers::normalize_answers(
                topic_id,
                panel.rows.iter().filter_map(|row| {
                    row.values
                        .get(column_index)
                        .map(|value| (row.label.as_str(), *value))
                }),
            );
            (!answers.is_empty()).then(|| DemographicResult {
                demographic: demographics::demographic_for_panel_column(panel, column_index),
                answers,
            })
        })
        .collect()
}

/// Wrap answers in a total demographic result if not empty.
fn non_empty_total(answers: Vec<AnswerResult>) -> Option<DemographicResult> {
    (!answers.is_empty()).then(|| DemographicResult {
        demographic: demographics::total_demographic(),
        answers,
    })
}

/// Remove duplicate demographics by ID.
fn dedupe_demographics(demographics: Vec<DemographicResult>) -> Vec<DemographicResult> {
    let mut seen = HashSet::new();
    demographics
        .into_iter()
        .filter(|demographic| seen.insert(demographic.demographic.id.clone()))
        .collect()
}
