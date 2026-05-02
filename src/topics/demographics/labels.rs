//! # Demographic label normalizer
//!
//! Normalizes demographic labels from polling data into canonical forms.

use super::groups::group_value;
use crate::sources::DataGroup;
use crate::topics::types::DemographicValue;

pub(super) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convert a label to a slug (lowercase, dashes).
pub(super) fn slug(input: &str) -> String {
    let mut output = String::new();
    let mut last_dash = false;

    for ch in input.to_ascii_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }

    output.trim_matches('-').to_string()
}

/// Map a demographic label to a canonical `DemographicValue`.
///
/// # Parameters
/// - `label`: The raw label.
/// - `group`: Optional group context.
pub(super) fn demographic_for_label(label: &str, group: Option<&DataGroup>) -> DemographicValue {
    let normalized = normalize_text(label);
    let lower = normalized.to_ascii_lowercase();
    let group_value = group_value(group);
    let group_id = group_value.as_ref().map(|group| group.id.as_str());

    let id = match (group_id, lower.as_str()) {
        (_, "total") | (_, "all adults") | (_, "all respondents") => "total".to_string(),
        (_, "registered voters") | (Some("registration"), "voters") => {
            "registered-voters".to_string()
        }
        (Some("party-id"), "dem" | "democrat" | "democratic") | (_, "democrat") => {
            "party-democrat".to_string()
        }
        (Some("party-id"), "rep" | "republican") | (_, "republican") => {
            "party-republican".to_string()
        }
        (Some("party-id"), "ind" | "independent" | "independent/something else")
        | (_, "independent")
        | (_, "independent/something else") => "party-independent".to_string(),
        (Some("vote-2024"), "harris") => "vote-2024-harris".to_string(),
        (Some("vote-2024"), "trump") => "vote-2024-trump".to_string(),
        (Some("gender"), "male") | (_, "male") => "gender-male".to_string(),
        (Some("gender"), "female") | (_, "female") => "gender-female".to_string(),
        (Some("race"), "white") | (_, "white") => "race-white".to_string(),
        (Some("race"), "black") | (_, "black") => "race-black".to_string(),
        (Some("race"), "hispanic") | (_, "hispanic") => "race-hispanic".to_string(),
        (Some("age"), "18-29") | (_, "18-29") => "age-18-29".to_string(),
        (Some("age"), "30-44") | (_, "30-44") => "age-30-44".to_string(),
        (Some("age"), "45-59") | (_, "45-59") => "age-45-59".to_string(),
        (Some("age"), "45-64") | (_, "45-64") => "age-45-64".to_string(),
        (Some("age"), "60+") | (_, "60+") => "age-60-plus".to_string(),
        (Some("age"), "65+") | (_, "65+") => "age-65-plus".to_string(),
        (Some("education"), "no degree") | (_, "no degree") => "education-no-degree".to_string(),
        (Some("education"), "college grad" | "college graduate") | (_, "college grad") => {
            "education-college-grad".to_string()
        }
        (Some("ideology"), "lib" | "liberal") => "ideology-liberal".to_string(),
        (Some("ideology"), "mod" | "moderate") => "ideology-moderate".to_string(),
        (Some("ideology"), "con" | "conservative") => "ideology-conservative".to_string(),
        (Some("maga"), "supporter") => "maga-supporter".to_string(),
        _ => format!("custom-{}", slug(&normalized)),
    };

    DemographicValue {
        id,
        label: normalized,
        group: group_value,
    }
}
