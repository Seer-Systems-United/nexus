use crate::sources::{DataCollection, DataPanel, DataStructure, Scope, Source};
use crate::topics::answers;
use crate::topics::catalog;
use crate::topics::demographics;
use crate::topics::enrichment;
use crate::topics::mappings::{self, TopicMatch};
use crate::topics::nlp;
use crate::topics::types::{
    AnswerResult, DemographicResult, DemographicValue, HeadlineTopicSummary, PooledAnswerResult,
    PooledDemographicResult, SourceId, TopicCollection, TopicObservation, TopicSource, TopicStatus,
    TopicSummary,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone)]
pub(crate) struct MappedSourceData {
    pub(crate) observations: Vec<TopicObservation>,
    pub(crate) warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct PooledValue {
    label: String,
    total: f32,
    count: usize,
}

pub async fn get_topic(scope: Scope, topic_id: &str) -> Result<TopicCollection, DynError> {
    let mapped = collect_mapped_source_data(scope).await;
    let observations = mapped
        .observations
        .into_iter()
        .filter(|observation| observation.topic_id == topic_id)
        .collect::<Vec<_>>();
    let topic = catalog::stable_topic(topic_id)
        .or_else(|| observations.first().map(observation_topic_summary))
        .ok_or_else(|| IoError::new(ErrorKind::NotFound, format!("topic not found: {topic_id}")))?;

    Ok(TopicCollection {
        topic,
        scope,
        pooled: pool_observations(&observations),
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
        if catalog::stable_topic(&observation.topic_id).is_some() {
            continue;
        }
        grouped
            .entry(observation.topic_id.clone())
            .or_default()
            .push(observation);
    }

    let mut summaries = grouped
        .into_values()
        .filter(|observations| unique_poll_count(observations) >= min_observations.max(1))
        .filter_map(|observations| headline_summary(&observations))
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
    let mut mapped = collect_unenriched_source_data(scope).await;

    match enrichment::apply_index_to_observations(&mut mapped.observations) {
        Ok(applied) => {
            if applied > 0 {
                tracing::info!(applied, "applied offline topic enrichment classifications");
            }
        }
        Err(error) => {
            mapped
                .warnings
                .push(format!("topic enrichment index unavailable: {error}"));
            tracing::warn!(error = %error, "failed to read topic enrichment index");
        }
    }

    nlp::cluster_headline_observations(&mut mapped.observations);

    mapped
}

pub(crate) async fn collect_unenriched_source_data(scope: Scope) -> MappedSourceData {
    let mut observations = Vec::new();
    let mut warnings = Vec::new();

    for source in SourceId::ALL {
        match load_source(source, scope).await {
            Ok(collection) => {
                observations.extend(map_source_collection(source, &collection));
            }
            Err(error) => {
                warnings.push(format!("{} unavailable: {error}", source.name()));
                tracing::warn!(
                    source = source.id(),
                    error = %error,
                    "failed to load topic source data"
                );
            }
        }
    }

    MappedSourceData {
        observations,
        warnings,
    }
}

async fn load_source(source: SourceId, scope: Scope) -> Result<DataCollection, DynError> {
    match source {
        SourceId::Emerson => crate::sources::emerson::Emerson::get_data(scope).await,
        SourceId::Gallup => crate::sources::gallup::Gallup::get_data(scope).await,
        SourceId::Ipsos => crate::sources::ipsos::Ipsos::get_data(scope).await,
        SourceId::YouGov => crate::sources::yougov::YouGov::get_data(scope).await,
    }
}

fn map_source_collection(source: SourceId, collection: &DataCollection) -> Vec<TopicObservation> {
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
    let (question_title, prompt) = mappings::common::structure_title_prompt(structure);
    let demographics = demographics_from_structure(&topic_match.topic.id, structure);

    if demographics.is_empty() {
        return None;
    }

    let poll_date = mappings::common::date_in_text(&format!(
        "{} {} {}",
        question_title,
        prompt,
        collection.subtitle.clone().unwrap_or_default()
    ));

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

fn demographics_from_structure(
    topic_id: &str,
    structure: &DataStructure,
) -> Vec<DemographicResult> {
    let demographics = match structure {
        DataStructure::Crosstab { panels, .. } => panels
            .iter()
            .flat_map(|panel| panel_demographics(topic_id, panel))
            .collect(),
        DataStructure::BarGraph { x, y, .. } => {
            let answers = answers::normalize_answers(
                topic_id,
                x.iter()
                    .zip(y.iter())
                    .map(|(label, value)| (label.as_str(), *value)),
            );
            non_empty_total(answers).into_iter().collect()
        }
        DataStructure::PieChart { slices, .. } => {
            let answers = answers::normalize_answers(
                topic_id,
                slices
                    .iter()
                    .map(|slice| (slice.label.as_str(), slice.value)),
            );
            non_empty_total(answers).into_iter().collect()
        }
        DataStructure::LineGraph { series, .. } => {
            let answers = answers::normalize_answers(
                topic_id,
                series.iter().filter_map(|series| {
                    series
                        .values
                        .last()
                        .map(|value| (series.label.as_str(), *value))
                }),
            );
            non_empty_total(answers).into_iter().collect()
        }
        DataStructure::Unstructured { .. } => Vec::new(),
    };

    dedupe_demographics(demographics)
}

fn panel_demographics(topic_id: &str, panel: &DataPanel) -> Vec<DemographicResult> {
    panel
        .columns
        .iter()
        .enumerate()
        .filter_map(|(column_index, _)| {
            let answers = answers::normalize_answers(
                topic_id,
                panel.rows.iter().filter_map(|row| {
                    row.values
                        .get(column_index)
                        .map(|value| (row.label.as_str(), *value))
                }),
            );

            if answers.is_empty() {
                return None;
            }

            Some(DemographicResult {
                demographic: demographics::demographic_for_panel_column(panel, column_index),
                answers,
            })
        })
        .collect()
}

fn non_empty_total(answers: Vec<AnswerResult>) -> Option<DemographicResult> {
    (!answers.is_empty()).then(|| DemographicResult {
        demographic: demographics::total_demographic(),
        answers,
    })
}

fn dedupe_demographics(demographics: Vec<DemographicResult>) -> Vec<DemographicResult> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();

    for demographic in demographics {
        if seen.insert(demographic.demographic.id.clone()) {
            deduped.push(demographic);
        }
    }

    deduped
}

fn pool_observations(observations: &[TopicObservation]) -> Vec<PooledDemographicResult> {
    let mut demographics_by_id: HashMap<String, DemographicValue> = HashMap::new();
    let mut values: HashMap<(String, String), PooledValue> = HashMap::new();

    for observation in observations {
        for demographic in &observation.demographics {
            demographics_by_id
                .entry(demographic.demographic.id.clone())
                .or_insert_with(|| demographic.demographic.clone());

            for answer in &demographic.answers {
                values
                    .entry((demographic.demographic.id.clone(), answer.id.clone()))
                    .and_modify(|pooled| {
                        pooled.total += answer.value;
                        pooled.count += 1;
                    })
                    .or_insert(PooledValue {
                        label: answer.label.clone(),
                        total: answer.value,
                        count: 1,
                    });
            }
        }
    }

    let mut by_demographic: HashMap<String, Vec<PooledAnswerResult>> = HashMap::new();
    for ((demographic_id, answer_id), pooled) in values {
        by_demographic
            .entry(demographic_id)
            .or_default()
            .push(PooledAnswerResult {
                id: answer_id,
                label: pooled.label,
                value: pooled.total / pooled.count as f32,
                observation_count: pooled.count,
            });
    }

    let mut pooled = by_demographic
        .into_iter()
        .filter_map(|(demographic_id, mut answers)| {
            let demographic = demographics_by_id.remove(&demographic_id)?;
            answers.sort_by(|left, right| left.id.cmp(&right.id));
            Some(PooledDemographicResult {
                demographic,
                answers,
            })
        })
        .collect::<Vec<_>>();

    pooled.sort_by(|left, right| {
        (left.demographic.id != "total")
            .cmp(&(right.demographic.id != "total"))
            .then_with(|| left.demographic.id.cmp(&right.demographic.id))
    });

    pooled
}

fn observation_topic_summary(observation: &TopicObservation) -> TopicSummary {
    TopicSummary {
        id: observation.topic_id.clone(),
        label: observation.topic_label.clone(),
        status: TopicStatus::Headline,
        description: None,
        endpoint: Some(format!("/api/v1/topics/{}", observation.topic_id)),
    }
}

fn headline_summary(observations: &[TopicObservation]) -> Option<HeadlineTopicSummary> {
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
        topic: TopicSummary {
            id: first.topic_id.clone(),
            label: first.topic_label.clone(),
            status: TopicStatus::Headline,
            description: None,
            endpoint: Some(format!("/api/v1/topics/{}", first.topic_id)),
        },
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

fn unique_poll_count(observations: &[TopicObservation]) -> usize {
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
