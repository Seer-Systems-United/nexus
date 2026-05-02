mod aliases;
mod phrases;
mod stopwords;

use super::models::{CandidateTerms, MIN_TERMS, Term};
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;

pub use stopwords::is_stopword;

pub fn extract_terms(text: &str) -> Option<CandidateTerms> {
    let focused = super::text::focus_question_text(text);
    let term_text = super::text::battery_item_text(&focused).unwrap_or_else(|| focused.clone());
    let normalized = super::text::normalized_search_text(&term_text);
    let normalized_for_intent = super::text::normalized_search_text(&focused);
    let stemmer = Stemmer::create(Algorithm::English);
    let mut terms = Vec::new();
    let mut seen = HashSet::new();
    let intent = super::intent::intent_from_text(&normalized_for_intent);

    for term in phrases::phrase_terms(&normalized) {
        push_term(&mut terms, &mut seen, term);
    }
    for token in super::text::token_regex()
        .find_iter(&normalized)
        .map(|match_| match_.as_str())
    {
        if let Some(term) = canonical_term(token, &stemmer) {
            push_term(&mut terms, &mut seen, term);
        }
    }

    if terms.len() == 1
        && let Some(intent_term) = intent.as_deref().and_then(aliases::term_for_intent)
    {
        push_term(&mut terms, &mut seen, intent_term);
    }
    if terms.len() < MIN_TERMS {
        return None;
    }

    let features = terms
        .iter()
        .map(|term| term.key.clone())
        .collect::<HashSet<_>>();
    let mut signature_terms = features.iter().cloned().collect::<Vec<_>>();
    signature_terms.sort();

    Some(CandidateTerms {
        terms,
        features,
        signature: signature_terms.join(" "),
        intent,
    })
}

fn canonical_term(token: &str, stemmer: &Stemmer) -> Option<Term> {
    if token.chars().all(|ch| ch.is_ascii_digit())
        || super::text::is_question_code(token)
        || is_stopword(token)
    {
        return None;
    }
    if let Some(term) = aliases::aliased_term(token) {
        return Some(term);
    }
    if token.len() < 3 {
        return None;
    }

    let key = stemmer.stem(token).to_string();
    if key.len() < 3 || is_stopword(&key) {
        return None;
    }

    Some(Term {
        key,
        label: aliases::display_label(token),
    })
}

pub fn term(key: &str, label: &str) -> Term {
    Term {
        key: key.to_string(),
        label: label.to_string(),
    }
}

fn push_term(terms: &mut Vec<Term>, seen: &mut HashSet<String>, term: Term) {
    if seen.insert(term.key.clone()) {
        terms.push(term);
    }
}
