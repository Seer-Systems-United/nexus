use rust_bert::pipelines::ner::NERModel;
use tracing::{debug, info, instrument, warn};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubjectType {
    Person,
    Organization,
    Location,
    Other,
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub text: String,
    pub subject_type: SubjectType,
}

pub fn init_subjects() {
    info!("initializing subjects module");
    info!("subjects module initialized");
}

fn label_to_subject_type(label: &str) -> SubjectType {
    match label {
        "I-PER" => SubjectType::Person,
        "I-ORG" => SubjectType::Organization,
        "I-LOC" => SubjectType::Location,
        other => {
            warn!(label = %other, "unrecognized entity label; mapping to SubjectType::Other");
            SubjectType::Other
        }
    }
}

#[instrument(level = "info", skip_all, fields(text_len = text.len()))]
pub fn extract_subjects(text: &str) -> Vec<Subject> {
    debug!(?text, "extracting subjects from text");

    info!("loading NER model");
    let ner = NERModel::new(Default::default()).expect("Unable to create NERModel");

    info!("running NER model");
    let output = ner.predict(&[text]);

    debug!(?output, "ner output");

    let mut subjects = Vec::new();
    let mut current_subject: Option<Subject> = None;

    for entity in output.into_iter().flatten() {
        debug!(label = %entity.label, word = %entity.word, "processing entity");

        // If the entity is a continuation of the current subject, append it
        if let Some(subject) = &mut current_subject {
            if label_to_subject_type(&entity.label) == subject.subject_type {
                subject.text.push(' ');
                subject.text.push_str(&entity.word);
                continue;
            } else {
                // Otherwise, finalize the current subject and start a new one
                debug!(?subject, "finalizing current subject");
                subjects.push(subject.clone());
                current_subject = None;
            }
        }

        let subject_type = label_to_subject_type(&entity.label);

        current_subject = Some(Subject {
            text: entity.word,
            subject_type,
        });
    }

    // Finalize any remaining subject
    if let Some(subject) = current_subject {
        debug!(?subject, "finalizing last subject");
        subjects.push(subject);
    }

    info!(count = subjects.len(), "subjects extracted");
    subjects
}
