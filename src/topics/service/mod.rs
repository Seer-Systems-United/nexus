//! # Topics service module
//!
//! Orchestrates topic data collection, enrichment, and pooling.
//! Provides the main API for retrieving canonical topic data.

mod collect;
mod headlines;
mod map;
mod pooling;

use crate::sources::Scope;
use crate::topics::catalog;
use crate::topics::enrichment;
use crate::topics::nlp;
use crate::topics::types::{HeadlineTopicSummary, TopicCollection, TopicObservation};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

/// Boxed dynamic error type for topic operations.
type DynError = Box<dyn Error + Send + Sync>;

/// Result of mapping source data to topic observations.
///
/// # Fields
/// - `observations`: Mapped topic observations.
/// - `warnings`: Non-fatal warnings (e.g., enrichment unavailable).
#[derive(Debug, Clone)]
pub struct MappedSourceData {
    pub observations: Vec<TopicObservation>,
    pub warnings: Vec<String>,
}

/// Get topic data by topic ID.
///
/// Collects source data, maps to observations, applies enrichment
/// and NLP clustering, then pools the results.
///
/// # Parameters
/// - `scope`: The query scope.
/// - `topic_id`: The topic ID to retrieve.
///
/// # Returns
/// - `Ok(TopicCollection)`: Pooled topic data with observations.
///
/// # Errors
/// - Returns an error if the topic is not found.
pub async fn get_topic(scope: Scope, topic_id: &str) -> Result<TopicCollection, DynError> {
    let mapped = collect_mapped_source_data(scope).await;
    let observations = mapped
        .observations
        .into_iter()
        .filter(|observation| observation.topic_id == topic_id)
        .collect::<Vec<_>>();
    let topic = catalog::stable_topic(topic_id)
        .or_else(|| {
            observations
                .first()
                .map(headlines::observation_topic_summary)
        })
        .ok_or_else(|| IoError::new(ErrorKind::NotFound, format!("topic not found: {topic_id}")))?;

    Ok(TopicCollection {
        topic,
        scope,
        pooled: pooling::pool_observations(&observations),
        observations,
        warnings: mapped.warnings,
    })
}

/// Get headline topics that recur across sources.
///
/// Groups observations by topic, filters by minimum poll count,
/// and applies NLP clustering for topic identification.
///
/// # Parameters
/// - `scope`: The query scope.
/// - `min_observations`: Minimum number of polls for a headline topic.
///
/// # Returns
/// - `Ok(Vec<HeadlineTopicSummary>)`: Sorted headline topics.
pub async fn headline_topics(
    scope: Scope,
    min_observations: usize,
) -> Result<Vec<HeadlineTopicSummary>, DynError> {
    let mapped = collect_mapped_source_data(scope).await;
    let mut grouped: HashMap<String, Vec<TopicObservation>> = HashMap::new();

    for observation in mapped.observations {
        if catalog::stable_topic(&observation.topic_id).is_none() {
            grouped
                .entry(observation.topic_id.clone())
                .or_default()
                .push(observation);
        }
    }

    let mut summaries = grouped
        .into_values()
        .filter(|observations| {
            headlines::unique_poll_count(observations) >= min_observations.max(1)
        })
        .filter_map(|observations| headlines::headline_summary(&observations))
        .collect::<Vec<_>>();

    summaries.sort_by(|left, right| {
        right
            .latest_date
            .cmp(&left.latest_date)
            .then_with(|| right.observation_count.cmp(&left.observation_count))
    });

    Ok(summaries)
}

/// Collect and map source data to topic observations.
///
/// Loads data from all sources, maps to observations,
/// applies enrichment index, and runs NLP clustering.
async fn collect_mapped_source_data(scope: Scope) -> MappedSourceData {
    let mut mapped = collect::collect_unenriched_source_data(scope).await;
    if let Err(error) = enrichment::apply_index_to_observations(&mut mapped.observations) {
        mapped
            .warnings
            .push(format!("topic enrichment index unavailable: {error}"));
        tracing::warn!(error = %error, "failed to read topic enrichment index");
    }
    nlp::cluster_headline_observations(&mut mapped.observations);
    mapped
}

/// Collect source data without enrichment applied.
///
/// Used for testing and when enrichment is not needed.
pub async fn collect_unenriched_source_data(scope: Scope) -> MappedSourceData {
    collect::collect_unenriched_source_data(scope).await
}
