use super::rows::parse_row;
use crate::sources::ipsos::server::extract::text::{
    is_noise_line, is_question_title, is_sample_size_line, next_label_has_sample_size,
    normalize_line, sample_size_end, sample_size_index,
};

pub(super) fn likely_header_start(lines: &[String], index: usize) -> bool {
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

pub(super) fn find_header_start(lines: &[String], question_index: usize) -> Option<usize> {
    let end = (question_index + 24).min(lines.len());

    (question_index + 1..end).find(|index| likely_header_start(lines, *index))
}

pub(super) fn parse_columns(lines: &[String], start: usize) -> Option<(Vec<String>, usize)> {
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
            push_column(&mut columns, &mut label_parts)?;
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

fn push_column(columns: &mut Vec<String>, label_parts: &mut Vec<String>) -> Option<()> {
    let label = normalize_line(&label_parts.join(" "));
    if label.is_empty() {
        return None;
    }
    columns.push(label);
    label_parts.clear();
    Some(())
}
