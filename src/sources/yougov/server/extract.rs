use crate::sources::{DataCollection, DataStructure};
use std::error::Error;

type DynError = Box<dyn Error + Send + Sync>;

const DOCUMENT_TITLE: &str = "The Economist/YouGov Poll";

fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_page_number(line: &str) -> bool {
    !line.is_empty() && line.chars().all(|ch| ch.is_ascii_digit())
}

fn is_question_title(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    if !matches!(chars.peek(), Some(ch) if ch.is_ascii_digit()) {
        return false;
    }
    while matches!(chars.peek(), Some(ch) if ch.is_ascii_digit()) {
        chars.next();
    }
    if matches!(chars.peek(), Some(ch) if ch.is_ascii_uppercase()) {
        chars.next();
    }
    matches!(chars.next(), Some('.')) && matches!(chars.next(), Some(' '))
}

fn is_table_of_contents_entry(line: &str) -> bool {
    line.contains(". .")
}

fn parse_percentage(token: &str) -> Option<f32> {
    token.strip_suffix('%')?.replace(',', "").parse().ok()
}

fn parse_row(line: &str, expected_values: usize) -> Option<crate::sources::DataRow> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }
    let mut value_count = 0usize;
    for token in tokens.iter().rev() {
        if parse_percentage(token).is_some() {
            value_count += 1;
        } else {
            break;
        }
    }
    if value_count != expected_values || value_count >= tokens.len() {
        return None;
    }
    let split_at = tokens.len() - value_count;
    let label = tokens[..split_at].join(" ");
    let values = tokens[split_at..]
        .iter()
        .filter_map(|token| parse_percentage(token))
        .collect::<Vec<_>>();
    Some(crate::sources::DataRow { label, values })
}

const PRIMARY_PANEL_HEADER: &str = "Sex Race Age Education";
const SECONDARY_PANEL_HEADER: &str = "2024 Vote Reg Ideology MAGA Party ID";

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

#[derive(Clone, Copy)]
struct PanelTemplate {
    header: &'static str,
    columns: &'static [&'static str],
    groups: &'static [(&'static str, &'static [&'static str])],
}

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

fn panel_template_for(line: &str) -> Option<PanelTemplate> {
    let normalized = normalize_line(line);
    PANEL_TEMPLATES
        .iter()
        .copied()
        .find(|template| normalized == template.header)
}

fn parse_panel(
    lines: &[String],
    start: usize,
    template: PanelTemplate,
) -> Option<(crate::sources::DataPanel, usize)> {
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

pub fn extract_document_header(
    lines: &[String],
) -> Result<(String, Option<String>), Box<dyn Error + Send + Sync>> {
    let mut non_empty = lines
        .iter()
        .filter(|line| !line.is_empty() && !is_page_number(line));
    let title = match non_empty.next() {
        Some(t) => t.clone(),
        None => return Err("missing poll document title".into()),
    };
    let subtitle = non_empty.next().cloned();
    Ok((title, subtitle))
}

fn parse_questions(lines: &[String]) -> Vec<DataStructure> {
    let mut questions = Vec::new();
    let mut index = 0usize;
    while index < lines.len() {
        let title_line = &lines[index];
        if !is_question_title(title_line) || is_table_of_contents_entry(title_line) {
            index += 1;
            continue;
        }
        let mut first_panel_start = None;
        for cursor in index + 1..(index + 20).min(lines.len()) {
            let line = &lines[cursor];
            if line == DOCUMENT_TITLE || (cursor > index + 1 && is_question_title(line)) {
                break;
            }
            if panel_template_for(line).is_some() {
                first_panel_start = Some(cursor);
                break;
            }
        }
        let Some(first_panel_start) = first_panel_start else {
            index += 1;
            continue;
        };
        let prompt = lines[index + 1..first_panel_start]
            .iter()
            .filter(|line| !line.is_empty() && !is_page_number(line))
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let mut panels = Vec::new();
        let mut cursor = first_panel_start;
        while cursor < lines.len() {
            let line = &lines[cursor];
            if line.is_empty()
                || is_page_number(line)
                || line == DOCUMENT_TITLE
                || line.contains("U.S. Adult Citizens")
            {
                cursor += 1;
                continue;
            }
            if let Some(template) = panel_template_for(line) {
                if let Some((panel, next_cursor)) = parse_panel(lines, cursor, template) {
                    panels.push(panel);
                    cursor = next_cursor;
                    continue;
                }
            }
            if cursor > first_panel_start && is_question_title(line) {
                break;
            }
            cursor += 1;
        }
        if !panels.is_empty() {
            questions.push(DataStructure::Crosstab {
                title: title_line.to_string(),
                prompt,
                panels,
                y_unit: "%".to_string(),
            });
            index = cursor;
            continue;
        }
        index += 1;
    }
    questions
}

pub fn extract_yougov_data(
    pdfs: &[Vec<u8>],
) -> Result<DataCollection, Box<dyn Error + Send + Sync>> {
    let mut all_data = Vec::new();
    let mut main_title = String::new();
    let mut main_subtitle = None;
    for (i, pdf_bytes) in pdfs.iter().enumerate() {
        let text = match pdf_extract::extract_text_from_mem(pdf_bytes) {
            Ok(text) => text,
            Err(_) => continue,
        };
        let lines = text.lines().map(normalize_line).collect::<Vec<_>>();
        let (title, subtitle) = match extract_document_header(&lines) {
            Ok(header) => header,
            Err(_) => continue,
        };
        if i == 0 {
            main_title = title;
            main_subtitle = subtitle.clone();
        }
        let prefix = subtitle.unwrap_or_else(|| "Unknown Date".to_string());
        let mut data = parse_questions(&lines);
        for structure in &mut data {
            match structure {
                DataStructure::BarGraph { title, .. }
                | DataStructure::LineGraph { title, .. }
                | DataStructure::PieChart { title, .. } => {
                    *title = format!("{}: {}", prefix, title);
                }
                DataStructure::Crosstab { title, prompt, .. } => {
                    *title = format!("{}: {}", prefix, title);
                    *prompt = format!("{}: {}", prefix, prompt);
                }
                DataStructure::Unstructured { .. } => {}
            }
        }
        all_data.extend(data);
    }
    if all_data.is_empty() {
        return Err("no poll questions found in Economist crosstabs PDF".into());
    }
    if main_title.is_empty() {
        main_title = "The Economist/YouGov Poll".to_string();
    }
    tracing::info!(
        source = "yougov",
        structures = all_data.len(),
        "extracted YouGov source data"
    );
    Ok(DataCollection {
        title: main_title,
        subtitle: main_subtitle,
        data: all_data,
    })
}

#[cfg(test)]
mod tests {
    use super::{is_question_title, parse_row};
    #[test]
    fn parses_question_titles() {
        assert!(is_question_title("1. Direction of Country"));
        assert!(is_question_title(
            "11A. Trump Approval on Issues — Foreign policy"
        ));
        assert!(!is_question_title("List of Tables"));
    }
    #[test]
    fn parses_rows_from_total_and_group_values() {
        let row = parse_row(
            "Generally headed in the right direction 32% 39% 25% 37% 17% 28% 27% 30% 33% 37% 34% 28%",
            12,
        )
        .expect("row should parse");
        assert_eq!(row.label, "Generally headed in the right direction");
        assert_eq!(row.values[0], 32.0);
        assert_eq!(row.values.len(), 12);
    }
}
