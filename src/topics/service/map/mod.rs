mod demographics;

use crate::sources::{DataCollection, DataStructure, SourceId};
use crate::topics::mappings::{self, TopicMatch};
use crate::topics::nlp;
use crate::topics::types::TopicObservation;

pub fn map_source_collection(
    source: SourceId,
    collection: &DataCollection,
) -> Vec<TopicObservation> {
    collection
        .data
        .iter()
        .enumerate()
        .filter_map(|(index, structure)| {
            let topic_match = mappings::match_structure(source, structure)
                .or_else(|| nlp::headline_candidate_match(source, structure))?;
            observation_from_structure(source, collection, index, structure, topic_match)
        })
        .collect()
}

fn observation_from_structure(
    source: SourceId,
    collection: &DataCollection,
    index: usize,
    structure: &DataStructure,
    topic_match: TopicMatch,
) -> Option<TopicObservation> {
    let (raw_question_title, raw_prompt) = mappings::common::structure_title_prompt(structure);
    let demographics = demographics::demographics_from_structure(&topic_match.topic.id, structure);
    if demographics.is_empty() {
        return None;
    }

    let poll_date = mappings::common::date_in_text(&format!(
        "{} {} {}",
        raw_question_title,
        raw_prompt,
        collection.subtitle.clone().unwrap_or_default()
    ));
    let question_title = nlp::clean_question_text(source, &raw_question_title);
    let prompt = nlp::clean_question_text(source, &raw_prompt);

    Some(TopicObservation {
        id: format!("{}:{}:{index}", source.id(), topic_match.topic.id),
        topic_id: topic_match.topic.id.clone(),
        topic_label: topic_match.topic.label.clone(),
        source: source.into(),
        source_collection: collection.title.clone(),
        source_subtitle: collection.subtitle.clone(),
        question_title,
        prompt,
        poll_date,
        compatibility: topic_match.compatibility,
        demographics,
    })
}
