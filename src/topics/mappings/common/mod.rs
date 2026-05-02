//! # Common mapping utilities
//!
//! Shared utilities for topic matching.
//! Handles text normalization and date extraction.

mod stable;

use crate::sources::DataStructure;

pub(crate) use stable::generic_match;

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
