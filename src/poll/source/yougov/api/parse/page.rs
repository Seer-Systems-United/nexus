use tracing::instrument;

use crate::poll::question::{Question, is_non_question_text};

use super::{
    question::build_question_text,
    response::{is_column_header_line, parse_responses_from_iter},
};

#[instrument(level = "debug", skip_all, fields(page_len = page.len()))]
pub fn parse_page(page: &str) -> Option<Question> {
    if is_non_question_text(page) {
        return None;
    }

    let mut lines = page
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    lines.next()?;
    lines.next()?;

    let mut question_lines = Vec::with_capacity(2);
    let mut first_column_header = None;

    for line in lines.by_ref() {
        if is_column_header_line(line) {
            first_column_header = Some(line);
            break;
        }

        question_lines.push(line);
    }

    let first_column_header = first_column_header?;
    let question = build_question_text(&question_lines)?;
    let responses = parse_responses_from_iter(first_column_header, &mut lines);

    if responses.is_empty() {
        None
    } else {
        Some(Question::new(&question, responses))
    }
}
