use crate::topics::enrichment::{ClassificationOutput, DynError, text::slug_id};
use std::collections::HashSet;
use std::io::{Error as IoError, ErrorKind};

pub fn parse_classifier_output(raw: &str) -> Result<ClassificationOutput, DynError> {
    let json_text = extract_json_object(raw).ok_or_else(|| {
        IoError::new(
            ErrorKind::InvalidData,
            "LLM response did not contain a JSON object",
        )
    })?;
    let mut output = serde_json::from_str::<ClassificationOutput>(json_text)?;

    output.confidence = normalize_confidence(output.confidence);
    output.canonical_topic_id = output.canonical_topic_id.trim().to_string();
    output.canonical_label = output.canonical_label.trim().to_string();
    output.intent = output.intent.trim().to_string();
    output.subject = normalized_subject(output.subject);
    output.exclude_reason = output
        .exclude_reason
        .map(|reason| reason.trim().to_string())
        .filter(|reason| !reason.is_empty());

    if output.canonical_topic_id.is_empty() && !output.canonical_label.is_empty() {
        output.canonical_topic_id = format!("headline-{}", slug_id(&output.canonical_label));
    }
    if output.canonical_topic_id.is_empty() && output.exclude_reason.is_none() {
        output.exclude_reason = Some("model returned no canonical topic id".to_string());
    }

    Ok(output)
}

fn extract_json_object(raw: &str) -> Option<&str> {
    let after_reasoning = raw.rsplit("</think>").next().unwrap_or(raw).trim();
    let start = after_reasoning.find('{')?;
    let end = after_reasoning.rfind('}')?;

    (start <= end).then(|| &after_reasoning[start..=end])
}

fn normalize_confidence(confidence: f32) -> f32 {
    let confidence = if confidence > 1.0 && confidence <= 100.0 {
        confidence / 100.0
    } else {
        confidence
    };

    confidence.clamp(0.0, 1.0)
}

fn normalized_subject(subject: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut normalized = Vec::new();

    for value in subject {
        let value = slug_id(&value);
        if !value.is_empty() && seen.insert(value.clone()) {
            normalized.push(value);
        }
    }

    normalized
}
