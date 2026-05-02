//! # Answer text utilities
//!
//! Slug generation and text normalization for answer labels.

/// Convert a string to a slug (lowercase, alphanumeric with dashes).
///
/// # Parameters
/// - `input`: The string to convert.
///
/// # Returns
/// - Slugified string.
pub fn slug(input: &str) -> String {
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

/// Normalize whitespace in text: collapse runs into single spaces.
pub fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Check if a label is a "net" answer or exact match for a root.
///
/// # Parameters
/// - `label`: The normalized answer label.
/// - `root`: The root word to match.
pub fn is_net_or_exact(label: &str, root: &str) -> bool {
    let lower = label.to_ascii_lowercase();
    lower == root || lower.contains("(net)") || lower.contains(" net")
}
