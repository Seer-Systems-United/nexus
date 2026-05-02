//! # Phrase matching
//!
//! Detects multi-word phrases in question text.
//! Uses regex patterns for common phrases.

use crate::topics::nlp::models::Term;
use regex::Regex;

pub fn phrase_terms(text: &str) -> Vec<Term> {
    let mut terms = Vec::new();

    if phrase_match(text, r"\b(artificial intelligence|generative ai|ai)\b") {
        terms.push(super::term("ai", "AI"));
    }
    if phrase_match(text, r"\bcost of living\b") {
        terms.push(super::term("price", "price"));
    }
    if phrase_match(text, r"\b(sports betting|online betting|online wagering)\b") {
        terms.push(super::term("sports", "sports"));
        terms.push(super::term("betting", "betting"));
    }
    if phrase_match(
        text,
        r"\b(military action|military strike|military strikes|air strike|air strikes|send troops|ground troops|war)\b",
    ) {
        terms.push(super::term("military-conflict", "military conflict"));
    }

    terms
}

fn phrase_match(text: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .expect("valid headline phrase regex")
        .is_match(text)
}
