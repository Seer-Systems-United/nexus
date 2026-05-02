//! # Weak feature identifiers
//!
//! Lists features that are too generic for clustering.

pub(super) fn is_weak_cluster_feature(feature: &str) -> bool {
    WEAK_FEATURES.contains(&feature)
}

/// Features that are too generic for meaningful clustering.
const WEAK_FEATURES: &[&str] = &[
    "approval",
    "attention-focus",
    "awareness",
    "benefit",
    "cost-benefit",
    "direction",
    "effect",
    "favorability",
    "future-effect",
    "institution",
    "impact",
    "issue",
    "local-condition",
    "people",
    "person",
    "policy",
    "price",
    "public",
    "safety-effect",
    "success-failure",
    "support-oppose",
    "vote-choice",
    "worth-it",
];
