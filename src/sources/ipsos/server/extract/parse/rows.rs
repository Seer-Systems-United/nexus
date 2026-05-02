use crate::sources::DataRow;
use crate::sources::ipsos::server::extract::text::{is_noise_line, is_question_title};
use crate::sources::ipsos::server::extract::text::{normalize_line, parse_value_token};

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

pub fn parse_row(line: &str, expected_values: usize) -> Option<DataRow> {
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    let values = trailing_values(&tokens, expected_values)?;
    let label_tokens = &tokens[..tokens.len().saturating_sub(expected_values)];
    let label = label_tokens.join(" ");

    if label.is_empty() || label.eq_ignore_ascii_case("total") {
        return None;
    }

    Some(DataRow { label, values })
}

pub(super) fn parse_rows(
    lines: &[String],
    start: usize,
    expected_values: usize,
) -> (Vec<DataRow>, usize) {
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
