//! # Text cleaning utilities
//!
//! Normalizes question text for different sources.
//! Strips metadata prefixes and question codes.

use crate::topics::types::SourceId;

/// Normalize text for use as a question key.
pub fn normalized_question_key(text: &str) -> String {
    super::normalized_search_text(&clean_question_text_generic(text))
}

pub fn clean_question_text(source: SourceId, text: &str) -> String {
    match source {
        SourceId::Ipsos | SourceId::YouGov => clean_source_question_text(text),
        SourceId::Emerson | SourceId::Gallup => {
            strip_leading_question_code(&normalize_spacing(text))
        }
    }
}

pub fn clean_question_text_generic(text: &str) -> String {
    let text = normalize_spacing(text);

    if let Some(cleaned) = strip_to_question_code(&text) {
        return cleaned;
    }

    let stripped = strip_ipsos_metadata_prefix(&strip_yougov_metadata_prefix(&text));
    if stripped != text {
        return strip_leading_question_code(&stripped);
    }

    if let Some((_, tail)) = text.rsplit_once(':')
        && tail.split_whitespace().count() >= 2
    {
        return strip_leading_question_code(tail);
    }

    strip_leading_question_code(&text)
}

fn clean_source_question_text(text: &str) -> String {
    let text = normalize_spacing(text);

    if let Some(cleaned) = strip_to_question_code(&text) {
        return cleaned;
    }

    let stripped = strip_ipsos_metadata_prefix(&strip_yougov_metadata_prefix(&text));
    if stripped != text {
        return strip_leading_question_code(&stripped);
    }

    strip_leading_question_code(&text)
}

fn normalize_spacing(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_to_question_code(text: &str) -> Option<String> {
    super::regexes::question_code_regex()
        .find(text)
        .map(|match_| strip_leading_question_code(&text[match_.start()..]))
}

fn strip_leading_question_code(text: &str) -> String {
    let trimmed = text.trim_start_matches(|ch: char| ch == ':' || ch.is_whitespace());

    if let Some(match_) = super::regexes::leading_question_code_regex().find(trimmed) {
        return trimmed[match_.end()..].trim().to_string();
    }

    trimmed.trim().to_string()
}

fn strip_ipsos_metadata_prefix(text: &str) -> String {
    super::regexes::ipsos_metadata_prefix_regex()
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|match_| match_.as_str().trim().to_string())
        .unwrap_or_else(|| text.to_string())
}

fn strip_yougov_metadata_prefix(text: &str) -> String {
    super::regexes::yougov_metadata_prefix_regex()
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|match_| match_.as_str().trim().to_string())
        .unwrap_or_else(|| text.to_string())
}
