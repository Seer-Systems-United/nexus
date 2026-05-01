use crate::sources::{DataGroup, DataPanel};
use crate::topics::types::{DemographicGroup, DemographicValue};

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn slug(input: &str) -> String {
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

fn group_for_column<'a>(panel: &'a DataPanel, column: &str) -> Option<&'a DataGroup> {
    panel
        .groups
        .iter()
        .find(|group| group.labels.iter().any(|label| label == column))
}

fn group_value(group: Option<&DataGroup>) -> Option<DemographicGroup> {
    let group = group?;
    let label = normalize_text(&group.title);

    if label.is_empty() {
        return None;
    }

    Some(DemographicGroup {
        id: match label.to_ascii_lowercase().as_str() {
            "sex" | "gender" => "gender".to_string(),
            "race" | "race/ethnicity" | "race/hispanic ethnicity" => "race".to_string(),
            "age" => "age".to_string(),
            "education" => "education".to_string(),
            "party" | "party id" | "political party" => "party-id".to_string(),
            "2024 vote" => "vote-2024".to_string(),
            "ideology" => "ideology".to_string(),
            "maga" => "maga".to_string(),
            "reg" | "registration" => "registration".to_string(),
            _ => slug(&label),
        },
        label,
    })
}

pub(crate) fn demographic_for_panel_column(
    panel: &DataPanel,
    column_index: usize,
) -> DemographicValue {
    let column = panel
        .columns
        .get(column_index)
        .map(|column| normalize_text(column))
        .unwrap_or_else(|| "Total".to_string());
    let group = group_for_column(panel, &column);
    demographic_for_label(&column, group)
}

pub(crate) fn total_demographic() -> DemographicValue {
    demographic_for_label("Total", None)
}

fn demographic_for_label(label: &str, group: Option<&DataGroup>) -> DemographicValue {
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

#[cfg(test)]
mod tests {
    use super::demographic_for_panel_column;
    use crate::sources::{DataGroup, DataPanel};

    #[test]
    fn maps_party_id_demographics_from_panel_groups() {
        let panel = DataPanel {
            columns: vec![
                "Total".to_string(),
                "Dem".to_string(),
                "Ind".to_string(),
                "Rep".to_string(),
            ],
            groups: vec![DataGroup {
                title: "Party ID".to_string(),
                labels: vec!["Dem".to_string(), "Ind".to_string(), "Rep".to_string()],
            }],
            rows: Vec::new(),
        };

        assert_eq!(demographic_for_panel_column(&panel, 0).id, "total");
        assert_eq!(demographic_for_panel_column(&panel, 1).id, "party-democrat");
        assert_eq!(
            demographic_for_panel_column(&panel, 2).id,
            "party-independent"
        );
        assert_eq!(
            demographic_for_panel_column(&panel, 3).id,
            "party-republican"
        );
    }
}
