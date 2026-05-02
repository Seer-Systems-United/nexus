use super::columns::{find_header_start, parse_columns};
use super::rows::parse_rows;
use crate::sources::ipsos::server::extract::text::{is_noise_line, is_question_title};
use crate::sources::{DataGroup, DataPanel, DataStructure};

pub fn parse_questions(lines: &[String]) -> Vec<DataStructure> {
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
        let title = question_title(lines, index, header_start);
        let Some((columns, row_start)) = parse_columns(lines, header_start) else {
            index += 1;
            continue;
        };
        let (rows, next_index) = parse_rows(lines, row_start, columns.len());

        if !rows.is_empty() {
            structures.push(crosstab(title, columns, rows));
            index = next_index;
            continue;
        }

        index += 1;
    }

    structures
}

fn question_title(lines: &[String], index: usize, header_start: usize) -> String {
    let title_line = &lines[index];
    let prompt = lines[index + 1..header_start]
        .iter()
        .filter(|line| !is_noise_line(line))
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    if prompt.is_empty() {
        title_line.clone()
    } else {
        format!("{title_line} {prompt}")
    }
}

fn crosstab(
    title: String,
    columns: Vec<String>,
    rows: Vec<crate::sources::DataRow>,
) -> DataStructure {
    DataStructure::Crosstab {
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
    }
}
