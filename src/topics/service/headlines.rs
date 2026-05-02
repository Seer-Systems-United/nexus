//! # Headline topic utilities
//!
//! Generates headline topic summaries and counts unique polls.
//! Helps identify recurring topics across sources.

use crate::topics::types::{
    HeadlineTopicSummary, TopicObservation, TopicSource, TopicStatus, TopicSummary,
};
use std::collections::HashSet;

/// Convert a single observation into a topic summary for headlines.
///
/// # Parameters
/// - `observation`: The observation to convert.
///
/// # Returns
/// - `TopicSummary` with headline status.
pub fn observation_topic_summary(observation: &TopicObservation) -> TopicSummary {
    TopicSummary {
        id: observation.topic_id.clone(),
        label: observation.topic_label.clone(),
        status: TopicStatus::Headline,
        description: None,
        endpoint: Some(format!("/api/v1/topics/{}", observation.topic_id)),
    }
}

/// Generate a headline topic summary from grouped observations.
///
/// # Parameters
/// - `observations`: Observations for a single headline topic.
///
/// # Returns
/// - `Some(HeadlineTopicSummary)` with aggregated metadata.
pub fn headline_summary(observations: &[TopicObservation]) -> Option<HeadlineTopicSummary> {
    let first = observations.first()?;
    let mut source_ids = HashSet::new();
    let mut sources = Vec::new();
    let mut sample_questions = Vec::new();

    for observation in observations {
        if source_ids.insert(observation.source.id.clone()) {
            sources.push(observation.source.clone());
        }
        if sample_questions.len() < 3 && !sample_questions.contains(&observation.question_title) {
            sample_questions.push(observation.question_title.clone());
        }
    }

    sources.sort_by(|left: &TopicSource, right: &TopicSource| left.id.cmp(&right.id));

    Some(HeadlineTopicSummary {
        topic: observation_topic_summary(first),
        observation_count: observations.len(),
        source_count: source_ids.len(),
        sources,
        latest_date: observations
            .iter()
            .filter_map(|observation| observation.poll_date.clone())
            .max(),
        sample_questions,
    })
}

/// Count unique polls (by source:collection:date) in observations.
///
/// # Parameters
/// - `observations`: The observations to count.
///
/// # Returns
/// - Number of unique polls.
pub fn unique_poll_count(observations: &[TopicObservation]) -> usize {
    observations
        .iter()
        .map(|observation| {
            format!(
                "{}:{}:{}",
                observation.source.id,
                observation.source_collection,
                observation.poll_date.as_deref().unwrap_or_default()
            )
        })
        .collect::<HashSet<_>>()
        .len()
}
