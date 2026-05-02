//! # Topics NLP tests
//!
//! Tests for natural language processing topic classification.

use nexus::sources::DataStructure;
use nexus::topics::nlp::{
    clean_question_text, cluster_headline_observations, headline_candidate_match,
    normalized_question_key, terms,
};
use nexus::topics::types::{Compatibility, SourceId, TopicObservation, TopicSource, TopicStatus};
#[test]
fn creates_dynamic_candidate_from_question_text() {
    let structure = DataStructure::BarGraph {
        title: "Do you support or oppose U.S. military strikes against Iran?".to_string(),
        x: vec!["Support".to_string(), "Oppose".to_string()],
        y: vec![45.0, 40.0],
        y_unit: "%".to_string(),
    };

    let topic = headline_candidate_match(SourceId::Emerson, &structure)
        .expect("question should produce dynamic headline candidate");

    assert_eq!(topic.topic.status, TopicStatus::Headline);
    assert!(topic.topic.id.starts_with("headline-candidate-"));
}
#[test]
fn clusters_same_support_question_across_wording() {
    let mut observations = vec![
        observation("Do you support or oppose the U.S. military action against Iran?"),
        observation("Support for Iran War"),
    ];

    cluster_headline_observations(&mut observations);
    assert_eq!(observations[0].topic_id, observations[1].topic_id);
}
#[test]
fn does_not_cluster_different_answer_intents() {
    let mut observations = vec![
        observation("Do you support U.S. military action against Iran?"),
        observation("Was it worth it for the United States to go to war with Iran?"),
    ];

    cluster_headline_observations(&mut observations);
    assert_ne!(observations[0].topic_id, observations[1].topic_id);
}
#[test]
fn extracts_salient_terms_without_poll_boilerplate() {
    let extracted = terms::extract_terms(
        "Latest Reuters/Ipsos poll: Americans increasingly support sports betting",
    )
    .expect("terms should parse");

    assert!(extracted.features.contains("sports"));
    assert!(extracted.features.contains("betting"));
    assert!(!extracted.features.contains("ipsos"));
    assert!(!extracted.features.contains("poll"));
}
#[test]
fn cleans_source_metadata_from_display_question_text() {
    assert_eq!(
        clean_question_text(
            SourceId::Ipsos,
            "2026-04-28: Americans feel the economy is on the wrong track: TM1128Y17_37. Do you approve?"
        ),
        "Do you approve?",
    );
    assert_eq!(
        clean_question_text(
            SourceId::YouGov,
            "April 24 - 27, 2026 - 1836 U.S. Adult Citizens: 15. Support for Iran War",
        ),
        "Support for Iran War",
    );
}
#[test]
fn normalized_question_keys_drop_source_metadata_and_codes() {
    assert_eq!(
        normalized_question_key(
            "2026-04-28: Americans feel the economy is on the wrong track: TM1128Y17_37. Do you approve?"
        ),
        "do you approve?",
    );
}
fn observation(question: &str) -> TopicObservation {
    TopicObservation {
        id: question.to_string(),
        topic_id: "headline-candidate-test".to_string(),
        topic_label: question.to_string(),
        source: TopicSource {
            id: "test".to_string(),
            name: "Test".to_string(),
        },
        source_collection: "Test".to_string(),
        source_subtitle: None,
        question_title: question.to_string(),
        prompt: question.to_string(),
        poll_date: None,
        compatibility: Compatibility::RollupCompatible,
        demographics: Vec::new(),
    }
}
