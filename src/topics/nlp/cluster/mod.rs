//! # Topic clustering module
//!
//! Clusters headline observations by topic similarity.
//! Uses term overlap and fuzzy matching.

mod similarity;

use super::models::{Candidate, Cluster};
use crate::topics::types::{Compatibility, TopicObservation};
use std::collections::HashSet;

pub fn cluster_headline_observations(observations: &mut [TopicObservation]) {
    let candidates = observations
        .iter()
        .enumerate()
        .filter(|(_, observation)| observation.topic_id.starts_with("headline-candidate-"))
        .filter_map(|(index, observation)| {
            let mut terms =
                super::terms::extract_terms(&super::text::observation_question_text(observation))?;
            terms.intent = super::intent::intent_from_observation(observation).or(terms.intent);
            Some(Candidate { index, terms })
        })
        .collect::<Vec<_>>();

    let clusters = cluster_candidates(candidates);
    apply_clusters(observations, clusters);
}

fn cluster_candidates(candidates: Vec<Candidate>) -> Vec<Cluster> {
    let mut clusters = Vec::new();

    for candidate in candidates {
        let best = clusters
            .iter()
            .enumerate()
            .map(|(index, cluster)| (index, similarity::cluster_similarity(&candidate, cluster)))
            .max_by(|(_, left), (_, right)| left.total_cmp(right));

        if let Some((index, _score)) = best.filter(|(_, score)| *score >= 0.45) {
            clusters[index].add(candidate);
        } else {
            clusters.push(Cluster::new(candidate));
        }
    }

    clusters
}

fn apply_clusters(observations: &mut [TopicObservation], clusters: Vec<Cluster>) {
    let mut used_ids = HashSet::new();

    for cluster in clusters {
        let label = super::labels::cluster_label(&cluster);
        let topic_id = unique_topic_id(&label, &cluster, &mut used_ids);

        for candidate in cluster.candidates {
            let observation = &mut observations[candidate.index];
            observation.topic_id = topic_id.clone();
            observation.topic_label = label.clone();
            observation.compatibility = Compatibility::RollupCompatible;
        }
    }
}

fn unique_topic_id(label: &str, cluster: &Cluster, used_ids: &mut HashSet<String>) -> String {
    let mut base = format!("headline-{}", super::slug(&label));
    if base == "headline-" {
        base = format!(
            "headline-{}",
            super::short_hash(&super::cluster_key(cluster))
        );
    }
    let mut topic_id = base.clone();
    let key = super::cluster_key(cluster);
    let mut salt = 0usize;

    while used_ids.contains(&topic_id) {
        let suffix_key = if salt == 0 {
            key.clone()
        } else {
            format!("{key}:{salt}")
        };
        topic_id = format!("{base}-{}", super::short_hash(&suffix_key));
        salt += 1;
    }

    used_ids.insert(topic_id.clone());
    topic_id
}
