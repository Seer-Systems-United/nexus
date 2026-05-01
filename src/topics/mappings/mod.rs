use crate::sources::DataStructure;
use crate::topics::types::{Compatibility, SourceId, TopicSummary};

pub(crate) mod common;
mod emerson;
mod gallup;
mod ipsos;
mod yougov;

#[derive(Debug, Clone)]
pub(crate) struct TopicMatch {
    pub topic: TopicSummary,
    pub compatibility: Compatibility,
}

pub(crate) fn match_structure(source: SourceId, structure: &DataStructure) -> Option<TopicMatch> {
    match source {
        SourceId::Emerson => emerson::match_structure(structure),
        SourceId::Gallup => gallup::match_structure(structure),
        SourceId::Ipsos => ipsos::match_structure(structure),
        SourceId::YouGov => yougov::match_structure(structure),
    }
}

pub(crate) fn stable_match(topic_id: &str, compatibility: Compatibility) -> Option<TopicMatch> {
    crate::topics::catalog::stable_topic(topic_id).map(|topic| TopicMatch {
        topic,
        compatibility,
    })
}
