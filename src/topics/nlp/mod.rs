//! # Natural Language Processing module
//!
//! NLP-based topic classification and clustering.
//! Extracts terms, clusters headline observations,
//! and generates canonical topic IDs and labels.

mod cluster;
mod intent;
mod labels;
mod models;
pub mod terms;
mod text;

use crate::sources::DataStructure;
use crate::topics::mappings::{self, TopicMatch};
use crate::topics::types::{Compatibility, SourceId, TopicStatus, TopicSummary};

pub use cluster::cluster_headline_observations;
pub use text::{clean_question_text, normalized_question_key};

pub fn headline_candidate_match(source: SourceId, structure: &DataStructure) -> Option<TopicMatch> {
    if !matches!(
        structure,
        DataStructure::BarGraph { .. }
            | DataStructure::LineGraph { .. }
            | DataStructure::PieChart { .. }
            | DataStructure::Crosstab { .. }
    ) {
        return None;
    }

    let text = mappings::common::structure_text(structure);
    if is_duplicate_source_question(source, &text) {
        return None;
    }

    let terms = terms::extract_terms(&text)?;
    let label = labels::label_from_terms(&terms.terms, None);
    let topic_id = format!("headline-candidate-{}", short_hash(&terms.signature));

    Some(TopicMatch {
        topic: TopicSummary {
            id: topic_id.clone(),
            label,
            status: TopicStatus::Headline,
            description: Some("Dynamically generated from poll question wording.".to_string()),
            endpoint: Some(format!("/api/v1/topics/{topic_id}")),
        },
        compatibility: Compatibility::RollupCompatible,
    })
}

fn is_duplicate_source_question(source: SourceId, text: &str) -> bool {
    matches!(source, SourceId::Ipsos)
        && (text.contains("approval5_1.") || text.contains("approval5_2."))
}

fn slug(input: &str) -> String {
    let mut output = String::new();
    let mut last_dash = false;

    for ch in input.to_ascii_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }

    output.trim_matches('-').to_string()
}

fn short_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{:08x}", hash as u32)
}

fn cluster_key(cluster: &models::Cluster) -> String {
    let mut terms = cluster.term_counts.keys().cloned().collect::<Vec<_>>();
    terms.sort();
    let mut intents = cluster
        .candidates
        .iter()
        .filter_map(|candidate| candidate.terms.intent.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    intents.sort();

    format!("{}|{}", terms.join("-"), intents.join("-"))
}
