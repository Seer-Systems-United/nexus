//! # Topic observation pooling
//!
//! Aggregates multiple topic observations by averaging
//! demographic answer values across sources.

use crate::topics::types::{
    DemographicValue, PooledAnswerResult, PooledDemographicResult, TopicObservation,
};
use std::collections::HashMap;

/// Pooled value for a specific answer label.
#[derive(Debug, Clone)]
struct PooledValue {
    label: String,
    total: f32,
    count: usize,
}

/// Pool observations by averaging answer values across sources.
///
/// # Parameters
/// - `observations`: Observations to pool.
///
/// # Returns
/// - `Vec<PooledDemographicResult>`: Pooled demographic results.
pub fn pool_observations(observations: &[TopicObservation]) -> Vec<PooledDemographicResult> {
    let mut demographics_by_id: HashMap<String, DemographicValue> = HashMap::new();
    let mut values: HashMap<(String, String), PooledValue> = HashMap::new();

    for observation in observations {
        for demographic in &observation.demographics {
            demographics_by_id
                .entry(demographic.demographic.id.clone())
                .or_insert_with(|| demographic.demographic.clone());

            for answer in &demographic.answers {
                values
                    .entry((demographic.demographic.id.clone(), answer.id.clone()))
                    .and_modify(|pooled| {
                        pooled.total += answer.value;
                        pooled.count += 1;
                    })
                    .or_insert(PooledValue {
                        label: answer.label.clone(),
                        total: answer.value,
                        count: 1,
                    });
            }
        }
    }

    sorted_pooled_results(demographics_by_id, values)
}

/// Convert pooled values into sorted demographic results.
fn sorted_pooled_results(
    mut demographics_by_id: HashMap<String, DemographicValue>,
    values: HashMap<(String, String), PooledValue>,
) -> Vec<PooledDemographicResult> {
    let mut by_demographic: HashMap<String, Vec<PooledAnswerResult>> = HashMap::new();
    for ((demographic_id, _answer_id), pooled) in values {
        by_demographic
            .entry(demographic_id)
            .or_default()
            .push(PooledAnswerResult {
                id: pooled.label.clone(),
                label: pooled.label,
                value: pooled.total / pooled.count as f32,
                observation_count: pooled.count,
            });
    }

    let mut pooled = by_demographic
        .into_iter()
        .filter_map(|(demographic_id, mut answers)| {
            let demographic = demographics_by_id.remove(&demographic_id)?;
            answers.sort_by(|left, right| left.id.cmp(&right.id));
            Some(PooledDemographicResult {
                demographic,
                answers,
            })
        })
        .collect::<Vec<_>>();

    pooled.sort_by(|left, right| {
        (left.demographic.id != "total")
            .cmp(&(right.demographic.id != "total"))
            .then_with(|| left.demographic.id.cmp(&right.demographic.id))
    });
    pooled
}
