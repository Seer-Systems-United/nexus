//! # Cluster label generation
//!
//! Generates human-readable labels from cluster terms.
//! Selects ordered terms and frequent terms.

use super::models::{Cluster, ClusterTerm, MAX_LABEL_TERMS, Term};
use std::collections::{HashMap, HashSet};

pub fn cluster_label(cluster: &Cluster) -> String {
    let first_terms = cluster
        .candidates
        .first()
        .map(|candidate| candidate.terms.terms.as_slice())
        .unwrap_or_default();

    let label = label_from_terms(first_terms, Some(&cluster.term_counts));
    if label.is_empty() {
        "Recent poll question".to_string()
    } else {
        label
    }
}

pub fn label_from_terms(
    ordered_terms: &[Term],
    term_counts: Option<&HashMap<String, ClusterTerm>>,
) -> String {
    let mut selected = select_ordered_terms(ordered_terms, term_counts);
    fill_with_frequent_terms(&mut selected, term_counts);

    selected
        .into_iter()
        .map(|term| title_term(&term))
        .collect::<Vec<_>>()
        .join(" ")
}

fn select_ordered_terms(
    ordered_terms: &[Term],
    term_counts: Option<&HashMap<String, ClusterTerm>>,
) -> Vec<String> {
    let mut selected = Vec::new();
    let mut seen = HashSet::new();

    for term in ordered_terms {
        let repeated = term_counts
            .and_then(|counts| counts.get(&term.key))
            .map(|term| term.count > 1)
            .unwrap_or(true);
        if (repeated || selected.is_empty()) && seen.insert(term.key.clone()) {
            selected.push(term.label.clone());
        }
        if selected.len() >= MAX_LABEL_TERMS {
            break;
        }
    }

    selected
}

fn fill_with_frequent_terms(
    selected: &mut Vec<String>,
    term_counts: Option<&HashMap<String, ClusterTerm>>,
) {
    let mut seen = selected.iter().cloned().collect::<HashSet<_>>();
    let Some(counts) = term_counts else {
        return;
    };
    let mut frequent = counts.iter().collect::<Vec<_>>();
    frequent.sort_by(|(left_key, left_term), (right_key, right_term)| {
        right_term
            .count
            .cmp(&left_term.count)
            .then_with(|| left_key.cmp(right_key))
    });

    for (key, term) in frequent {
        if selected.len() >= MAX_LABEL_TERMS {
            break;
        }
        if seen.insert(key.clone()) {
            selected.push(term.label.clone());
        }
    }
}

fn title_term(term: &str) -> String {
    match term {
        "AI" | "COVID" | "U.S." => term.to_string(),
        _ => title_words(term),
    }
}
fn title_words(term: &str) -> String {
    term.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
