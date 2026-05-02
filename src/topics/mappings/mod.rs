//! # Topic mappings module
//!
//! Maps source-specific topics to canonical Nexus topics.

use crate::sources::DataStructure;
use crate::topics::types::{Compatibility, SourceId, TopicSummary};

pub(crate) mod common;
mod sources;

#[derive(Debug, Clone)]
pub struct TopicMatch {
    pub topic: TopicSummary,
    pub compatibility: Compatibility,
}

pub(crate) fn match_structure(source: SourceId, structure: &DataStructure) -> Option<TopicMatch> {
    match source {
        SourceId::Emerson => sources::emerson::match_structure(structure),
        SourceId::Gallup => sources::gallup::match_structure(structure),
        SourceId::Ipsos => sources::ipsos::match_structure(structure),
        SourceId::YouGov => sources::yougov::match_structure(structure),
    }
}

pub(crate) fn stable_match(topic_id: &str, compatibility: Compatibility) -> Option<TopicMatch> {
    crate::topics::catalog::stable_topic(topic_id).map(|topic| TopicMatch {
        topic,
        compatibility,
    })
}
