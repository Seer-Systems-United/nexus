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

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct MappedSourceData {
    pub observations: Vec<TopicObservation>,
    pub warnings: Vec<String>,
}

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

pub async fn collect_unenriched_source_data(scope: Scope) -> MappedSourceData {
    collect::collect_unenriched_source_data(scope).await
}
