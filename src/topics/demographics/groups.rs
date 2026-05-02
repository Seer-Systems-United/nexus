use super::labels::{normalize_text, slug};
use crate::sources::{DataGroup, DataPanel};
use crate::topics::types::DemographicGroup;

pub(super) fn group_for_column<'a>(panel: &'a DataPanel, column: &str) -> Option<&'a DataGroup> {
    panel
        .groups
        .iter()
        .find(|group| group.labels.iter().any(|label| label == column))
}

pub(super) fn group_value(group: Option<&DataGroup>) -> Option<DemographicGroup> {
    let group = group?;
    let label = normalize_text(&group.title);

    if label.is_empty() {
        return None;
    }

    Some(DemographicGroup {
        id: group_id(&label),
        label,
    })
}

fn group_id(label: &str) -> String {
    match label.to_ascii_lowercase().as_str() {
        "sex" | "gender" => "gender".to_string(),
        "race" | "race/ethnicity" | "race/hispanic ethnicity" => "race".to_string(),
        "age" => "age".to_string(),
        "education" => "education".to_string(),
        "party" | "party id" | "political party" => "party-id".to_string(),
        "2024 vote" => "vote-2024".to_string(),
        "ideology" => "ideology".to_string(),
        "maga" => "maga".to_string(),
        "reg" | "registration" => "registration".to_string(),
        _ => slug(label),
    }
}
