//! # YouGov template definitions and panel parser
//!
//! Defines panel templates for Economist/YouGov crosstabs
//! and provides parsing functions for panel data.

mod definitions;

use super::utils::*;
use definitions::PANEL_TEMPLATES;

/// A template for parsing a crosstab panel.
///
/// # Fields
/// - `header`: The header line that identifies this template.
/// - `columns`: Column labels for this panel.
/// - `groups`: Group definitions (title + labels).
#[derive(Clone, Copy)]
pub(crate) struct PanelTemplate {
    pub(crate) header: &'static str,
    pub(crate) columns: &'static [&'static str],
    pub(crate) groups: &'static [(&'static str, &'static [&'static str])],
}

/// Find a panel template that matches a given line.
///
/// # Parameters
/// - `line`: The line to match against templates.
///
/// # Returns
/// - `Some(PanelTemplate)` if a match is found.
pub(crate) fn panel_template_for(line: &str) -> Option<PanelTemplate> {
    let normalized = normalize_line(line);
    PANEL_TEMPLATES
        .iter()
        .copied()
        .find(|template| normalized == template.header)
}

/// Parse a panel starting at the given line.
///
/// # Parameters
/// - `lines`: All lines from the PDF text.
/// - `start`: The line index where the panel header starts.
///
/// # Returns
/// - `Some((DataPanel, next_cursor))` if parsing succeeds.
pub(crate) fn parse_panel(
    lines: &[String],
    start: usize,
) -> Option<(crate::sources::DataPanel, usize)> {
    let template = panel_template_for(lines.get(start)?)?;
    let mut cursor = start + 1;
    while cursor < lines.len() && lines[cursor].is_empty() {
        cursor += 1;
    }

    let column_line = lines.get(cursor)?;
    if normalize_line(column_line) != template.columns.join(" ") {
        return None;
    }
    cursor += 1;

    let mut rows = Vec::new();
    let mut pending = Vec::new();

    while cursor < lines.len() {
        let line = lines[cursor].as_str();
        if line.is_empty() {
            cursor += 1;
            continue;
        }
        if line.starts_with("Totals ") {
            cursor += 1;
            while cursor < lines.len()
                && (lines[cursor].is_empty() || lines[cursor].starts_with("Unweighted N"))
            {
                cursor += 1;
            }
            break;
        }
        if panel_template_for(line).is_some()
            || is_question_title(line)
            || line == DOCUMENT_TITLE
            || line.contains("U.S. Adult Citizens")
            || is_page_number(line)
        {
            break;
        }

        pending.push(line.to_string());
        let combined = pending.join(" ");
        if let Some(row) = parse_row(&combined, template.columns.len()) {
            rows.push(row);
            pending.clear();
        }
        cursor += 1;
    }

    Some((panel_from_template(template, rows), cursor))
}

/// Build a `DataPanel` from a template and parsed rows.
fn panel_from_template(
    template: PanelTemplate,
    rows: Vec<crate::sources::DataRow>,
) -> crate::sources::DataPanel {
    crate::sources::DataPanel {
        columns: template
            .columns
            .iter()
            .map(|column| (*column).to_string())
            .collect(),
        groups: template
            .groups
            .iter()
            .map(|(title, labels)| crate::sources::DataGroup {
                title: (*title).to_string(),
                labels: labels.iter().map(|label| (*label).to_string()).collect(),
            })
            .collect(),
        rows,
    }
}
