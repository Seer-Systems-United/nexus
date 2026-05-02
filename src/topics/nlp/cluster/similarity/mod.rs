//! # Cluster similarity module
//!
//! Computes similarity scores between topic candidates.
//! Uses Jaro-Winkler and set intersection.

mod features;

use super::super::models::{Candidate, CandidateTerms, Cluster};
use features::is_weak_cluster_feature;
use std::collections::HashSet;
use strsim::jaro_winkler;

pub fn cluster_similarity(candidate: &Candidate, cluster: &Cluster) -> f32 {
    cluster
        .candidates
        .iter()
        .filter_map(|existing| similarity_score(&candidate.terms, &existing.terms))
        .fold(0.0, f32::max)
}

fn similarity_score(left: &CandidateTerms, right: &CandidateTerms) -> Option<f32> {
    if let (Some(left_intent), Some(right_intent)) = (&left.intent, &right.intent)
        && left_intent != right_intent
    {
        return None;
    }

    let shared = left.features.intersection(&right.features).count();
    if shared == 0 {
        return None;
    }
    let salient_shared = left
        .features
        .intersection(&right.features)
        .filter(|feature| !is_weak_cluster_feature(feature))
        .count();
    let left_salient = salient_features(&left.features);
    let right_salient = salient_features(&right.features);

    if salient_shared == 0 && left.signature != right.signature {
        return None;
    }
    if (left.intent.is_some() || right.intent.is_some()) && left.intent != right.intent {
        return None;
    }
    if has_specificity_mismatch(&left_salient, &right_salient) {
        return None;
    }

    score_shared_terms(left, right, shared)
}

fn score_shared_terms(left: &CandidateTerms, right: &CandidateTerms, shared: usize) -> Option<f32> {
    let union = left.features.union(&right.features).count();
    let smaller = left.features.len().min(right.features.len());
    if union == 0 || smaller == 0 {
        return None;
    }

    let jaccard = shared as f32 / union as f32;
    let containment = shared as f32 / smaller as f32;
    let fuzzy = jaro_winkler(&left.signature, &right.signature) as f32;

    if shared >= 2 && (jaccard >= 0.25 || containment >= 0.50 || fuzzy >= 0.86) {
        return Some(jaccard.max(containment * 0.8).max(fuzzy * 0.65));
    }
    if shared == 1 && fuzzy >= 0.92 {
        return Some(fuzzy * 0.55);
    }

    None
}

fn salient_features(features: &HashSet<String>) -> HashSet<&str> {
    features
        .iter()
        .map(String::as_str)
        .filter(|feature| !is_weak_cluster_feature(feature))
        .collect()
}

fn has_specificity_mismatch(left: &HashSet<&str>, right: &HashSet<&str>) -> bool {
    if left.is_empty() || right.is_empty() || left == right {
        return false;
    }
    let (smaller, larger) = if left.len() <= right.len() {
        (left, right)
    } else {
        (right, left)
    };
    smaller.len() <= 2 && smaller.is_subset(larger)
}
