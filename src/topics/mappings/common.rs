use crate::sources::DataStructure;
use crate::topics::catalog::{
    ECONOMY_APPROVAL_ID, FOREIGN_POLICY_APPROVAL_ID, GENERIC_BALLOT_ID, IMMIGRATION_APPROVAL_ID,
    IMPORTANT_PROBLEM_ID, INFLATION_APPROVAL_ID, PRESIDENTIAL_APPROVAL_ID, RIGHT_DIRECTION_ID,
    TRUMP_FAVORABILITY_ID,
};
use crate::topics::mappings::{TopicMatch, stable_match};
use crate::topics::types::Compatibility;

pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn normalized_key(text: &str) -> String {
    normalize_text(text).to_ascii_lowercase()
}

pub(crate) fn structure_title_prompt(structure: &DataStructure) -> (String, String) {
    match structure {
        DataStructure::BarGraph { title, .. }
        | DataStructure::LineGraph { title, .. }
        | DataStructure::PieChart { title, .. } => (title.clone(), title.clone()),
        DataStructure::Crosstab { title, prompt, .. } => (title.clone(), prompt.clone()),
        DataStructure::Unstructured { data } => {
            let title = data.lines().next().unwrap_or_default().to_string();
            (title.clone(), title)
        }
    }
}

pub(crate) fn structure_text(structure: &DataStructure) -> String {
    let (title, prompt) = structure_title_prompt(structure);
    normalized_key(&format!("{title} {prompt}"))
}

pub(crate) fn date_in_text(text: &str) -> Option<String> {
    for token in text.split(|ch: char| !ch.is_ascii_digit() && ch != '-') {
        if token.len() == 10 {
            let mut parts = token.split('-');
            let year = parts.next()?;
            let month = parts.next()?;
            let day = parts.next()?;
            if year.len() == 4
                && month.len() == 2
                && day.len() == 2
                && year.chars().all(|ch| ch.is_ascii_digit())
                && month.chars().all(|ch| ch.is_ascii_digit())
                && day.chars().all(|ch| ch.is_ascii_digit())
            {
                return Some(token.to_string());
            }
        }
    }

    None
}

pub(crate) fn generic_match(structure: &DataStructure) -> Option<TopicMatch> {
    let text = structure_text(structure);

    if (text.contains("approve or disapprove")
        || text.contains("approval")
        || text.contains("job performance"))
        && (text.contains("president")
            || text.contains("donald trump")
            || text.contains("trump approval"))
        && !text.contains("issue?")
        && !text.contains("approval on issues")
        && !text.contains("handling the u.s. economy")
        && !text.contains("handling immigration")
        && !text.contains("handling the following issues")
        && !text.contains("handling the following things")
        && !text.contains("handling the situation in iran")
    {
        return stable_match(PRESIDENTIAL_APPROVAL_ID, Compatibility::EquivalentWording);
    }

    if text.contains("right direction") && text.contains("wrong track") {
        return stable_match(RIGHT_DIRECTION_ID, Compatibility::ExactWording);
    }

    if (text.contains("u.s. congress")
        || text.contains("congressional")
        || text.contains("generic ballot"))
        && text.contains("democratic")
        && text.contains("republican")
    {
        return stable_match(GENERIC_BALLOT_ID, Compatibility::EquivalentWording);
    }

    if text.contains("most important problem") || text.contains("main problem facing") {
        return stable_match(IMPORTANT_PROBLEM_ID, Compatibility::EquivalentWording);
    }

    if text.contains("handling the u.s. economy")
        || text.contains("handling the economy")
        || text.contains("u.s. economy")
    {
        return stable_match(ECONOMY_APPROVAL_ID, Compatibility::EquivalentWording);
    }

    if text.contains("inflation")
        || text.contains("rising prices")
        || text.contains("cost of living")
    {
        return stable_match(INFLATION_APPROVAL_ID, Compatibility::EquivalentWording);
    }

    if text.contains("immigration") && (text.contains("approve") || text.contains("handling")) {
        return stable_match(IMMIGRATION_APPROVAL_ID, Compatibility::EquivalentWording);
    }

    if text.contains("foreign policy") && (text.contains("approve") || text.contains("handling")) {
        return stable_match(FOREIGN_POLICY_APPROVAL_ID, Compatibility::EquivalentWording);
    }

    if text.contains("donald trump") && text.contains("favorable") && text.contains("unfavorable") {
        return stable_match(TRUMP_FAVORABILITY_ID, Compatibility::EquivalentWording);
    }

    None
}
