use std::sync::Arc;

use crate::poll::response::{Response, unit::Unit};

use super::{
    cell::split_response_line,
    columns::{ColumnSpec, demographic_for_column, parse_column_specs},
};

pub(super) fn parse_responses_from_iter<'a, I>(
    first_header: &'a str,
    lines: &mut I,
) -> Vec<Response>
where
    I: Iterator<Item = &'a str>,
{
    let mut responses = Vec::with_capacity(128);
    let mut columns = None;
    let mut pending_answer = None;
    let mut pending_values = Vec::with_capacity(16);

    for line in std::iter::once(first_header).chain(lines.by_ref()) {
        if is_column_header_line(line) {
            pending_answer = None;
            pending_values.clear();
            columns = parse_column_specs(line);
            continue;
        }

        let Some(active_columns) = columns.as_ref() else {
            continue;
        };

        if is_ignored_response_line(line) || is_response_artifact_line(line) {
            continue;
        }

        let (answer, values) = split_response_line(line);

        if let Some(answer) = answer {
            pending_answer = Some(Arc::from(answer));
            pending_values.clear();
        }

        if pending_answer.is_none() || values.is_empty() {
            continue;
        }

        pending_values.extend(values);

        if pending_values.len() >= active_columns.len() {
            push_response_row(
                pending_answer.take().expect("pending answer exists"),
                &pending_values,
                active_columns,
                &mut responses,
            );
            pending_values.clear();
        }
    }

    responses
}

pub(super) fn is_column_header_line(line: &str) -> bool {
    line.starts_with("Total ")
}

fn push_response_row(
    answer: Arc<str>,
    values: &[(u16, Unit)],
    columns: &[ColumnSpec],
    responses: &mut Vec<Response>,
) {
    for (column, (cell_value, unit)) in columns.iter().zip(values.iter()) {
        responses.push(Response {
            demographic: demographic_for_column(column),
            answer: Arc::clone(&answer),
            value: *cell_value,
            unit: unit.clone(),
        });
    }
}

fn is_ignored_response_line(line: &str) -> bool {
    let line = line.trim();

    line == "Totals"
        || line.starts_with("Totals ")
        || line.starts_with("Unweighted N")
        || line.starts_with('(')
        || is_page_number(line)
}

fn is_response_artifact_line(line: &str) -> bool {
    matches!(
        line.trim(),
        "Sex Race"
            | "Age"
            | "Education"
            | "Gender Age Ideology"
            | "Party ID Race White by Education"
            | "Reg"
            | "2024 Vote"
            | "Ideology"
            | "MAGA"
            | "Party"
            | "ID"
            | "Lib Mod"
            | "Voters"
            | "Con"
            | "Rep"
            | "Supporter Dem Ind"
    )
}

fn is_page_number(line: &str) -> bool {
    line.chars().all(|c| c.is_ascii_digit())
}
