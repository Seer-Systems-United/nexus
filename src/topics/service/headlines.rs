use crate::topics::types::{
    HeadlineTopicSummary, TopicObservation, TopicSource, TopicStatus, TopicSummary,
};
use std::collections::HashSet;

pub fn observation_topic_summary(observation: &TopicObservation) -> TopicSummary {
    TopicSummary {
        id: observation.topic_id.clone(),
        label: observation.topic_label.clone(),
        status: TopicStatus::Headline,
        description: None,
        endpoint: Some(format!("/api/v1/topics/{}", observation.topic_id)),
    }
}

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
