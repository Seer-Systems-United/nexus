use super::utils::*;

pub(crate) const PRIMARY_PANEL_HEADER: &str = "Sex Race Age Education";
pub(crate) const SECONDARY_PANEL_HEADER: &str = "2024 Vote Reg Ideology MAGA Party ID";

const PRIMARY_PANEL_COLUMNS: &[&str] = &[
    "Total",
    "Male",
    "Female",
    "White",
    "Black",
    "Hispanic",
    "18-29",
    "30-44",
    "45-64",
    "65+",
    "No degree",
    "College grad",
];

const SECONDARY_PANEL_COLUMNS: &[&str] = &[
    "Total",
    "Harris",
    "Trump",
    "Voters",
    "Lib",
    "Mod",
    "Con",
    "Supporter",
    "Dem",
    "Ind",
    "Rep",
];

const PRIMARY_PANEL_GROUPS: &[(&str, &[&str])] = &[
    ("Sex", &["Male", "Female"]),
    ("Race", &["White", "Black", "Hispanic"]),
    ("Age", &["18-29", "30-44", "45-64", "65+"]),
    ("Education", &["No degree", "College grad"]),
];

const SECONDARY_PANEL_GROUPS: &[(&str, &[&str])] = &[
    ("2024 Vote", &["Harris", "Trump"]),
    ("Reg", &["Voters"]),
    ("Ideology", &["Lib", "Mod", "Con"]),
    ("MAGA", &["Supporter"]),
    ("Party ID", &["Dem", "Ind", "Rep"]),
];

const PANEL_TEMPLATES: &[PanelTemplate] = &[
    PanelTemplate {
        header: PRIMARY_PANEL_HEADER,
        columns: PRIMARY_PANEL_COLUMNS,
        groups: PRIMARY_PANEL_GROUPS,
    },
    PanelTemplate {
        header: SECONDARY_PANEL_HEADER,
        columns: SECONDARY_PANEL_COLUMNS,
        groups: SECONDARY_PANEL_GROUPS,
    },
];

#[derive(Clone, Copy)]
pub(crate) struct PanelTemplate {
    pub(crate) header: &'static str,
    pub(crate) columns: &'static [&'static str],
    pub(crate) groups: &'static [(&'static str, &'static [&'static str])],
}

pub(crate) fn panel_template_for(line: &str) -> Option<PanelTemplate> {
    let normalized = normalize_line(line);
    PANEL_TEMPLATES
        .iter()
        .copied()
        .find(|template| normalized == template.header)
}

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

    Some((
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
        },
        cursor,
    ))
}
