//! # Topics enrichment tests
//!
//! Tests for topic enrichment classification and parsing.

use nexus::sources::Scope;
use nexus::topics::enrichment::{
    QuestionEnrichment, applicable_topic_id, parse_classifier_output, parse_scope,
};

#[test]
fn parses_classifier_json_after_reasoning_text() {
    let output = parse_classifier_output(
        r#"<think>not part of the answer</think>
        ```json
        {
          "canonical_topic_id": "iran military action support",
          "canonical_label": "Iran Military Action Support",
          "intent": "support_oppose",
          "subject": ["Iran", "Military Action"],
          "confidence": 84,
          "exclude_reason": null
        }
        ```"#,
    )
    .unwrap();

    assert_eq!(output.canonical_topic_id, "iran military action support");
    assert_eq!(output.subject, vec!["iran", "military-action"]);
    assert!((output.confidence - 0.84).abs() < f32::EPSILON);
}

#[test]
fn applies_only_accepted_or_stable_topic_ids() {
    let headline_record = record("iran military action support", 0.55);
    assert_eq!(
        applicable_topic_id(&headline_record).as_deref(),
        Some("headline-iran-military-action-support")
    );

    let stable = record("presidential-approval", 0.55);
    assert_eq!(
        applicable_topic_id(&stable).as_deref(),
        Some("presidential-approval")
    );

    let low_confidence = record("iran military action support", 0.54);
    assert!(applicable_topic_id(&low_confidence).is_none());
}

#[test]
fn parses_default_enrichment_scope() {
    assert_eq!(parse_scope(None, None).unwrap(), Scope::LastNEntries(5));
    assert_eq!(
        parse_scope(Some("last_days"), Some(30)).unwrap(),
        Scope::LastDays(30)
    );
}

fn record(topic_id: &str, confidence: f32) -> QuestionEnrichment {
    QuestionEnrichment {
        question_fingerprint: "fingerprint".to_string(),
        source: "source".to_string(),
        poll_date: None,
        source_collection: "collection".to_string(),
        question_title: "question".to_string(),
        prompt: "prompt".to_string(),
        answer_labels: Vec::new(),
        canonical_topic_id: topic_id.to_string(),
        canonical_label: String::new(),
        intent: "support_oppose".to_string(),
        subject: Vec::new(),
        confidence,
        model: "test".to_string(),
        review_status: "accepted".to_string(),
        exclude_reason: None,
    }
}
