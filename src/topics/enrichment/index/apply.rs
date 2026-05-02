use super::input::observation_fingerprint;
use super::storage::load_index;
use crate::topics::catalog;
use crate::topics::enrichment::{
    DynError, MIN_APPLY_CONFIDENCE, QuestionEnrichment,
    text::{label_from_topic_id, slug_id},
};
use crate::topics::types::{Compatibility, TopicObservation};
use std::collections::HashMap;

pub fn apply_index_to_observations(
    observations: &mut [TopicObservation],
) -> Result<usize, DynError> {
    let index = load_index()?;
    if index.records.is_empty() {
        return Ok(0);
    }

    let records = index
        .records
        .iter()
        .map(|record| (record.question_fingerprint.as_str(), record))
        .collect::<HashMap<_, _>>();
    let mut applied = 0usize;

    for observation in observations
        .iter_mut()
        .filter(|observation| observation.topic_id.starts_with("headline-candidate-"))
    {
        let fingerprint = observation_fingerprint(observation);
        let Some(record) = records.get(fingerprint.as_str()) else {
            continue;
        };
        let Some(topic_id) = applicable_topic_id(record) else {
            continue;
        };

        observation.topic_id = topic_id.clone();
        observation.topic_label = applicable_label(record, &topic_id);
        observation.compatibility = Compatibility::RollupCompatible;
        applied += 1;
    }

    Ok(applied)
}

pub fn applicable_topic_id(record: &QuestionEnrichment) -> Option<String> {
    if record.exclude_reason.is_some() || record.confidence < MIN_APPLY_CONFIDENCE {
        return None;
    }

    let raw_id = if record.canonical_topic_id.trim().is_empty() {
        record.canonical_label.trim()
    } else {
        record.canonical_topic_id.trim()
    };
    let mut topic_id = slug_id(raw_id);

    if topic_id.is_empty() {
        return None;
    }
    if catalog::stable_topic(&topic_id).is_some() {
        return Some(topic_id);
    }
    if !topic_id.starts_with("headline-") {
        topic_id = format!("headline-{topic_id}");
    }

    Some(topic_id)
}

fn applicable_label(record: &QuestionEnrichment, topic_id: &str) -> String {
    let label = record.canonical_label.trim();
    if !label.is_empty() {
        return label.to_string();
    }

    label_from_topic_id(topic_id)
}
