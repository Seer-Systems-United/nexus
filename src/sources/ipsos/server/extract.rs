use crate::sources::{DataCollection, DataGroup, DataPanel, DataRow, DataStructure, Scope};
use std::error::Error;

type DynError = Box<dyn Error + Send + Sync>;

const COMMON_NOISE_LINES: &[&str] = &[
    "TOPLINE & METHODOLOGY",
    "2020 K Street, NW, Suite 410",
    "Washington DC 20006",
    "+1 202 463-7300",
    "Contact:",
    "Email:",
    "Annotated Questionnaire",
];

fn normalize_line(line: &str) -> String {
    let cleaned = line
        .replace("TOPLINE & METHODOLOGY", "")
        .replace("Annotated Questionnaire", "");

    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_noise_line(line: &str) -> bool {
    line.is_empty()
        || line.chars().all(|ch| ch.is_ascii_digit())
        || COMMON_NOISE_LINES
            .iter()
            .any(|noise| line.eq_ignore_ascii_case(noise))
}

fn parse_value_token(token: &str) -> Option<f32> {
    let normalized = token
        .trim()
        .trim_matches(|ch: char| ch == ',' || ch == ';')
        .trim_start_matches('<')
        .trim_end_matches('%');

    if normalized == "*" || normalized == "-" || normalized.eq_ignore_ascii_case("n/a") {
        return Some(0.0);
    }

    normalized.replace(',', "").parse().ok()
}

fn trailing_values(tokens: &[&str], expected_values: usize) -> Option<Vec<f32>> {
    if tokens.len() <= expected_values {
        return None;
    }

    let values = tokens
        .iter()
        .rev()
        .take(expected_values)
        .map(|token| parse_value_token(token))
        .collect::<Option<Vec<_>>>()?;

    Some(values.into_iter().rev().collect())
}

fn parse_row(line: &str, expected_values: usize) -> Option<DataRow> {
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    let values = trailing_values(&tokens, expected_values)?;
    let label_tokens = &tokens[..tokens.len().saturating_sub(expected_values)];
    let label = label_tokens.join(" ");

    if label.is_empty() || label.eq_ignore_ascii_case("total") {
        return None;
    }

    Some(DataRow { label, values })
}

fn sample_size_index(line: &str) -> Option<usize> {
    line.find("(N=").or_else(|| line.find("N="))
}

fn sample_size_end(line: &str, sample_index: usize) -> usize {
    line[sample_index..]
        .find(')')
        .map(|offset| sample_index + offset + 1)
        .unwrap_or(line.len())
}

fn is_sample_size_line(line: &str) -> bool {
    sample_size_index(line).is_some()
}

fn is_question_title(line: &str) -> bool {
    let Some((prefix, rest)) = line.split_once(". ") else {
        return false;
    };

    let prefix = prefix.trim();
    !prefix.is_empty()
        && prefix.len() <= 32
        && prefix
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
        && rest.chars().any(|ch| ch == '?' || ch.is_ascii_alphabetic())
}

fn likely_header_start(lines: &[String], index: usize) -> bool {
    let Some(line) = lines.get(index) else {
        return false;
    };

    if line == "Total" {
        return lines
            .iter()
            .skip(index + 1)
            .take(12)
            .any(|line| is_sample_size_line(line));
    }

    line.starts_with("Total ") && is_sample_size_line(line)
}

fn find_header_start(lines: &[String], question_index: usize) -> Option<usize> {
    let end = (question_index + 24).min(lines.len());

    (question_index + 1..end).find(|index| likely_header_start(lines, *index))
}

fn next_label_has_sample_size(lines: &[String], start: usize) -> bool {
    for line in lines.iter().skip(start).take(8) {
        if is_noise_line(line) {
            continue;
        }
        if is_sample_size_line(line) {
            return true;
        }
        if parse_value_token(line.split_whitespace().last().unwrap_or_default()).is_some()
            || is_question_title(line)
        {
            return false;
        }
    }

    false
}

fn parse_columns(lines: &[String], start: usize) -> Option<(Vec<String>, usize)> {
    let mut columns = Vec::new();
    let mut label_parts = Vec::new();
    let mut cursor = start;

    while cursor < lines.len() {
        let line = &lines[cursor];
        if is_noise_line(line) {
            cursor += 1;
            continue;
        }

        if let Some(sample_index) = sample_size_index(line) {
            let sample_end = sample_size_end(line, sample_index);
            let inline_label = normalize_line(&line[..sample_index]);
            let next_inline_label = normalize_line(&line[sample_end..]);
            if !inline_label.is_empty() {
                label_parts.push(inline_label);
            }

            let label = normalize_line(&label_parts.join(" "));
            if label.is_empty() {
                return None;
            }

            columns.push(label);
            label_parts.clear();
            if !next_inline_label.is_empty() {
                label_parts.push(next_inline_label);
            }
            cursor += 1;

            if !next_label_has_sample_size(lines, cursor) {
                break;
            }
            continue;
        }

        if !columns.is_empty() && parse_row(line, columns.len()).is_some() {
            break;
        }

        if is_question_title(line) {
            break;
        }

        label_parts.push(line.clone());
        cursor += 1;

        if label_parts.len() > 4 {
            return None;
        }
    }

    (!columns.is_empty()).then_some((columns, cursor))
}

fn parse_rows(lines: &[String], start: usize, expected_values: usize) -> (Vec<DataRow>, usize) {
    let mut rows = Vec::new();
    let mut pending = Vec::new();
    let mut cursor = start;

    while cursor < lines.len() {
        let line = &lines[cursor];

        if is_noise_line(line) {
            cursor += 1;
            continue;
        }

        if is_question_title(line) {
            break;
        }

        pending.push(line.clone());
        let combined = normalize_line(&pending.join(" "));

        if let Some(row) = parse_row(&combined, expected_values) {
            rows.push(row);
            pending.clear();
        }

        cursor += 1;
    }

    (rows, cursor)
}

fn parse_questions(lines: &[String]) -> Vec<DataStructure> {
    let mut structures = Vec::new();
    let mut index = 0usize;

    while index < lines.len() {
        let title_line = &lines[index];
        if !is_question_title(title_line) {
            index += 1;
            continue;
        }

        let Some(header_start) = find_header_start(lines, index) else {
            index += 1;
            continue;
        };
        let prompt = lines[index + 1..header_start]
            .iter()
            .filter(|line| !is_noise_line(line))
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let title = if prompt.is_empty() {
            title_line.clone()
        } else {
            format!("{title_line} {prompt}")
        };

        let Some((columns, row_start)) = parse_columns(lines, header_start) else {
            index += 1;
            continue;
        };
        let (rows, next_index) = parse_rows(lines, row_start, columns.len());

        if !rows.is_empty() {
            structures.push(DataStructure::Crosstab {
                title: title.clone(),
                prompt: title,
                panels: vec![DataPanel {
                    columns: columns.clone(),
                    groups: vec![DataGroup {
                        title: "Respondents".to_string(),
                        labels: columns,
                    }],
                    rows,
                }],
                y_unit: "%".to_string(),
            });
            index = next_index;
            continue;
        }

        index += 1;
    }

    structures
}

fn prefix_structure(structure: &mut DataStructure, prefix: &str) {
    match structure {
        DataStructure::BarGraph { title, .. }
        | DataStructure::LineGraph { title, .. }
        | DataStructure::PieChart { title, .. } => {
            *title = format!("{prefix}: {title}");
        }
        DataStructure::Crosstab { title, prompt, .. } => {
            *title = format!("{prefix}: {title}");
            *prompt = format!("{prefix}: {prompt}");
        }
        DataStructure::Unstructured { data } => {
            *data = format!("{prefix}\n\n{data}");
        }
    }
}

fn collection_subtitle(
    scope: Scope,
    pdfs: &[crate::sources::ipsos::server::IpsosPollPdf],
) -> Option<String> {
    let first = pdfs.first()?;
    let last = pdfs.last().unwrap_or(first);
    let poll_label = if pdfs.len() == 1 { "poll" } else { "polls" };

    Some(format!(
        "{} collection: {} to {} ({} {poll_label})",
        scope.collection_label(),
        last.published_on,
        first.published_on,
        pdfs.len()
    ))
}

pub(crate) fn extract_ipsos_data(
    pdfs: &[crate::sources::ipsos::server::IpsosPollPdf],
    scope: Scope,
) -> Result<DataCollection, DynError> {
    let mut data = Vec::new();
    let mut pdf_failures = 0usize;
    let mut fallback_count = 0usize;

    for pdf in pdfs {
        let text = match pdf_extract::extract_text_from_mem(&pdf.bytes) {
            Ok(text) => text,
            Err(error) => {
                pdf_failures += 1;
                tracing::warn!(
                    source = "ipsos",
                    article_url = %pdf.article_url,
                    pdf_url = %pdf.pdf_url,
                    error = %error,
                    "failed to extract Ipsos PDF text"
                );
                continue;
            }
        };
        let lines = text
            .lines()
            .map(normalize_line)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let mut structures = parse_questions(&lines);
        let prefix = format!("{}: {}", pdf.published_on, pdf.title);

        if structures.is_empty() {
            fallback_count += 1;
            structures.push(DataStructure::Unstructured { data: text });
        }

        for structure in &mut structures {
            prefix_structure(structure, &prefix);
        }

        data.extend(structures);
    }

    if data.is_empty() {
        return Err("no Ipsos poll data found in PDFs".into());
    }

    tracing::info!(
        source = "ipsos",
        scope = %scope,
        structures = data.len(),
        pdf_failures,
        fallback_count,
        "extracted Ipsos source data"
    );

    Ok(DataCollection {
        title: "Ipsos Polls".to_string(),
        subtitle: collection_subtitle(scope, pdfs),
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::{is_question_title, normalize_line, parse_questions, parse_row};
    use crate::sources::DataStructure;

    #[test]
    fn parses_ipsos_question_titles() {
        assert!(is_question_title(
            "CP1. In your opinion, what is the most important problem facing the U.S. today?"
        ));
        assert!(is_question_title(
            "Approval5_1. Overall, do you approve or disapprove?"
        ));
        assert!(!is_question_title("www.ipsos.com"));
    }

    #[test]
    fn parses_rows_with_asterisk_and_dash_values() {
        let row = parse_row("Skipped 1% 1% - *", 4).expect("row should parse");

        assert_eq!(row.label, "Skipped");
        assert_eq!(row.values, vec![1.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn parses_ipsos_crosstab_from_pdf_lines() {
        let text = r#"
            CP2. Generally speaking, would you say things in this country are heading in the right direction, or are
            they off on the wrong track?
            Total
            (N=1,269)
            Republican
            (N=435)
            Democrat
            (N=351)
            Independent/Something
            else
            (N=451)
            Right direction 19% 46% 4% 11%
            Wrong track 64% 33% 90% 72%
            Don’t know 16% 21% 6% 17%
            Skipped 1% 1% - *
        "#;
        let lines = text.lines().map(normalize_line).collect::<Vec<_>>();
        let structures = parse_questions(&lines);

        assert_eq!(structures.len(), 1);
        let DataStructure::Crosstab { title, panels, .. } = &structures[0] else {
            panic!("expected crosstab");
        };

        assert!(title.starts_with("CP2."));
        assert_eq!(
            panels[0].columns,
            vec![
                "Total",
                "Republican",
                "Democrat",
                "Independent/Something else"
            ]
        );
        assert_eq!(panels[0].rows[1].label, "Wrong track");
        assert_eq!(panels[0].rows[1].values, vec![64.0, 33.0, 90.0, 72.0]);
    }

    #[test]
    fn parses_inline_total_sample_size_header() {
        let text = r#"
            Q1. Do you approve?
            Total (N=1,000)
            Republican (N=400)
            Democrat (N=350)
            Approve 40% 70% 10%
            Disapprove 55% 25% 85%
        "#;
        let lines = text.lines().map(normalize_line).collect::<Vec<_>>();
        let structures = parse_questions(&lines);

        assert_eq!(structures.len(), 1);
        let DataStructure::Crosstab { panels, .. } = &structures[0] else {
            panic!("expected crosstab");
        };

        assert_eq!(panels[0].columns, vec!["Total", "Republican", "Democrat"]);
        assert_eq!(panels[0].rows[0].values, vec![40.0, 70.0, 10.0]);
    }

    #[test]
    fn parses_sample_size_line_with_next_column_label() {
        let text = r#"
            CP2. Generally speaking, would you say things in this country are heading in the right direction, or are
            they off on the wrong track?
            Total
            (N=1,269) Republican
            (N=435) Democrat
            (N=351)
            Independent/Something
            else
            (N=451)
            Right direction 19% 46% 4% 11%
            Wrong track 64% 33% 90% 72%
        "#;
        let lines = text.lines().map(normalize_line).collect::<Vec<_>>();
        let structures = parse_questions(&lines);

        assert_eq!(structures.len(), 1);
        let DataStructure::Crosstab { panels, .. } = &structures[0] else {
            panic!("expected crosstab");
        };

        assert_eq!(
            panels[0].columns,
            vec![
                "Total",
                "Republican",
                "Democrat",
                "Independent/Something else"
            ]
        );
    }
}
