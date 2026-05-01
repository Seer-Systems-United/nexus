use super::templates::*;
use super::utils::*;

pub(crate) fn parse_questions(lines: &[String]) -> Vec<crate::sources::DataStructure> {
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

            if panel_template_for(line).is_some() {
                if let Some((panel, next_cursor)) = parse_panel(lines, cursor) {
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
            questions.push(crate::sources::DataStructure::Crosstab {
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
