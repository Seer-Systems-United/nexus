use crate::topics::enrichment::{ClassificationInput, QuestionEnrichment, text::short_hash};
use crate::topics::nlp;
use crate::topics::types::TopicObservation;
use std::collections::{HashMap, HashSet};

pub(in crate::topics::enrichment) fn classification_inputs(
    observations: &[TopicObservation],
    indexed_records: &HashMap<String, QuestionEnrichment>,
    refresh: bool,
) -> Vec<ClassificationInput> {
    let mut seen = HashSet::new();
    let mut inputs = Vec::new();

    for observation in observations {
        if !observation.topic_id.starts_with("headline-candidate-") {
            continue;
        }

        let input = classification_input_from_observation(observation);
        if !refresh && indexed_records.contains_key(&input.question_fingerprint) {
            continue;
        }
        if seen.insert(input.question_fingerprint.clone()) {
            inputs.push(input);
        }
    }

    inputs.sort_by(|left, right| left.question_fingerprint.cmp(&right.question_fingerprint));
    inputs
}

fn classification_input_from_observation(observation: &TopicObservation) -> ClassificationInput {
    ClassificationInput {
        question_fingerprint: observation_fingerprint(observation),
        source: observation.source.id.clone(),
        poll_date: observation.poll_date.clone(),
        source_collection: observation.source_collection.clone(),
        question_title: observation.question_title.clone(),
        prompt: observation.prompt.clone(),
        answer_labels: answer_labels_from_observation(observation),
    }
}

pub(super) fn observation_fingerprint(observation: &TopicObservation) -> String {
    let question_key = nlp::normalized_question_key(&observation_question_text(observation));
    let answers = answer_labels_from_observation(observation).join("|");
    short_hash(&format!("v1|{question_key}|{answers}"))
}

fn observation_question_text(observation: &TopicObservation) -> String {
    let title = observation.question_title.trim();
    let prompt = observation.prompt.trim();

    if prompt.is_empty() || title.eq_ignore_ascii_case(prompt) || title.contains(prompt) {
        title.to_string()
    } else if title.is_empty() {
        prompt.to_string()
    } else {
        format!("{title}: {prompt}")
    }
}

fn answer_labels_from_observation(observation: &TopicObservation) -> Vec<String> {
    let mut answers = HashSet::new();

    for demographic in &observation.demographics {
        for answer in &demographic.answers {
            answers.insert(format!("{}: {}", answer.id, answer.label));
        }
    }

    let mut answers = answers.into_iter().collect::<Vec<_>>();
    answers.sort();
    answers
}
