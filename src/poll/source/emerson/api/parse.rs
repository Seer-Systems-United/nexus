use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    sync::Arc,
};

use csv::StringRecord;

use crate::poll::{
    question::Question,
    response::{Response, demographic::Demographic, unit::Unit},
};

#[derive(Debug)]
struct ColumnAnswer {
    question: String,
    answer: String,
    count_index: usize,
    percent_index: usize,
}

struct RowGroup {
    question: String,
    answers: Vec<String>,
}

pub fn parse_full_crosstabs(csv: &str) -> Vec<Question> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv.as_bytes());

    let records = reader
        .records()
        .filter_map(Result::ok)
        .collect::<Vec<StringRecord>>();

    let Some(headers) = records.first() else {
        return Vec::new();
    };

    let row_groups = parse_row_groups(records.iter().skip(1));
    let columns = parse_column_answers(headers, &row_groups);
    if columns.is_empty() {
        return Vec::new();
    }

    let mut responses_by_question = BTreeMap::<String, Vec<Response>>::new();
    let mut question_order = Vec::<String>::new();
    let mut seen = HashSet::<String>::new();
    let mut current_row_question = String::new();

    for record in records.iter().skip(1) {
        let row_question = clean_text(record.get(0).unwrap_or_default());
        if !row_question.is_empty() {
            current_row_question = row_question;
        }

        let row_answer = clean_text(record.get(1).unwrap_or_default());
        if row_answer.is_empty() {
            continue;
        }

        let demographic = demographic_for_row(&current_row_question, &row_answer);

        for column in &columns {
            if let Some(value) = record.get(column.count_index).and_then(parse_count) {
                push_response(
                    &mut responses_by_question,
                    &mut question_order,
                    &mut seen,
                    column,
                    demographic.clone(),
                    value,
                    Unit::Count,
                );
            }

            if let Some(value) = record
                .get(column.percent_index)
                .and_then(parse_percent_tenths)
            {
                push_response(
                    &mut responses_by_question,
                    &mut question_order,
                    &mut seen,
                    column,
                    demographic.clone(),
                    value,
                    Unit::Other("percent_tenths".to_string()),
                );
            }
        }
    }

    question_order
        .into_iter()
        .filter_map(|question| {
            responses_by_question
                .remove(&question)
                .map(|responses| Question::new(&question, responses))
        })
        .collect()
}

fn parse_row_groups<'a, I>(records: I) -> Vec<RowGroup>
where
    I: Iterator<Item = &'a StringRecord>,
{
    let mut groups = Vec::new();
    let mut current_question = String::new();
    let mut answers = Vec::new();

    for record in records {
        let row_question = clean_text(record.get(0).unwrap_or_default());
        if !row_question.is_empty() {
            current_question = row_question;
            answers.clear();
        }

        let row_answer = clean_text(record.get(1).unwrap_or_default());
        if current_question.is_empty() || row_answer.is_empty() {
            continue;
        }

        answers.push(row_answer.clone());

        if row_answer.eq_ignore_ascii_case("total") {
            groups.push(RowGroup {
                question: current_question.clone(),
                answers: answers.clone(),
            });
            current_question.clear();
            answers.clear();
        }
    }

    groups
}

fn parse_column_answers(headers: &StringRecord, row_groups: &[RowGroup]) -> Vec<ColumnAnswer> {
    let mut columns = Vec::new();
    let mut group = Vec::<(usize, usize, String)>::new();
    let mut index = 2;

    while index < headers.len() {
        let header = clean_text(headers.get(index).unwrap_or_default());
        let Some(label) = header.strip_suffix(" Count") else {
            index += 1;
            continue;
        };

        let label = clean_text(label);
        group.push((index, index + 1, label.clone()));

        if label.eq_ignore_ascii_case("total") || label.ends_with(" Total") {
            let Some(row_group) = match_row_group(&group, row_groups) else {
                group.clear();
                index += 2;
                continue;
            };

            let question = row_group.question.clone();
            for (count_index, percent_index, label) in group.drain(..) {
                let answer = answer_for_label(&label, row_group);

                if answer.is_empty() || answer.eq_ignore_ascii_case("total") {
                    continue;
                }

                columns.push(ColumnAnswer {
                    question: question.clone(),
                    answer,
                    count_index,
                    percent_index,
                });
            }
        }

        index += 2;
    }

    columns
}

fn match_row_group<'a>(
    group: &[(usize, usize, String)],
    row_groups: &'a [RowGroup],
) -> Option<&'a RowGroup> {
    let first_label = group.first().map(|(_, _, label)| label.as_str())?;

    row_groups
        .iter()
        .filter(|row_group| {
            row_group
                .answers
                .first()
                .map(|first_answer| {
                    let expected = format!("{} {}", row_group.question, first_answer);
                    first_label == expected
                })
                .unwrap_or(false)
        })
        .max_by_key(|row_group| row_group.question.len())
}

fn answer_for_label(label: &str, row_group: &RowGroup) -> String {
    if let Some(first_answer) = row_group.answers.first() {
        if label == format!("{} {}", row_group.question, first_answer) {
            return first_answer.clone();
        }
    }

    clean_text(label)
}

fn demographic_for_row(row_question: &str, row_answer: &str) -> Demographic {
    if row_answer.eq_ignore_ascii_case("total") {
        Demographic::All
    } else if row_question.is_empty() {
        Demographic::Other {
            description: Cow::Owned(row_answer.to_string()),
        }
    } else {
        Demographic::Other {
            description: Cow::Owned(format!("{row_question}: {row_answer}")),
        }
    }
}

fn push_response(
    responses_by_question: &mut BTreeMap<String, Vec<Response>>,
    question_order: &mut Vec<String>,
    seen: &mut HashSet<String>,
    column: &ColumnAnswer,
    demographic: Demographic,
    value: u16,
    unit: Unit,
) {
    let key = format!(
        "{}|{}|{:?}|{}|{}",
        column.question,
        column.answer,
        demographic,
        value,
        unit_name(&unit)
    );

    if !seen.insert(key) {
        return;
    }

    if !responses_by_question.contains_key(&column.question) {
        question_order.push(column.question.clone());
    }

    responses_by_question
        .entry(column.question.clone())
        .or_default()
        .push(Response {
            demographic,
            answer: Arc::from(column.answer.as_str()),
            value,
            unit,
        });
}

fn parse_count(value: &str) -> Option<u16> {
    let value = clean_number(value);
    if value.is_empty() {
        return None;
    }

    value.parse::<u16>().ok().or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .and_then(|value| u16::try_from(value.round() as i64).ok())
    })
}

fn parse_percent_tenths(value: &str) -> Option<u16> {
    let value = clean_number(value.trim_end_matches('%'));
    if value.is_empty() {
        return None;
    }

    value
        .parse::<f64>()
        .ok()
        .and_then(|value| u16::try_from((value * 10.0).round() as i64).ok())
}

fn clean_number(value: &str) -> String {
    value.trim().replace(',', "")
}

fn clean_text(value: &str) -> String {
    value
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace('\u{a0}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn unit_name(unit: &Unit) -> &str {
    match unit {
        Unit::Other(name) => name.as_str(),
        Unit::Percent => "percent",
        Unit::Count => "count",
    }
}

#[cfg(test)]
mod tests {
    use super::parse_full_crosstabs;
    use crate::poll::response::{demographic::Demographic, unit::Unit};

    #[test]
    fn parses_emerson_full_crosstabs() {
        let csv = r#""","","Question A Yes Count","Row N %","No Count","Row N %","Total Count","Row N %","Question B Approve Count","Row N %","Disapprove Count","Row N %","Total Count","Row N %"
"Question A","Yes","10","100.0%","0","0.0%","10","100.0%","6","60.0%","4","40.0%","10","100.0%"
"","No","0","0.0%","20","100.0%","20","100.0%","5","25.0%","15","75.0%","20","100.0%"
"","Total","10","33.3%","20","66.7%","30","100.0%","11","36.7%","19","63.3%","30","100.0%"
"Question B","Approve","6","54.5%","5","45.5%","11","100.0%","11","100.0%","0","0.0%","11","100.0%"
"","Disapprove","4","21.1%","15","78.9%","19","100.0%","0","0.0%","19","100.0%","19","100.0%"
"","Total","10","33.3%","20","66.7%","30","100.0%","11","36.7%","19","63.3%","30","100.0%""#;

        let questions = parse_full_crosstabs(csv);

        assert_eq!(questions.len(), 2);
        assert_eq!(questions[0].text, "Question A");
        assert!(
            questions[0]
                .responses
                .iter()
                .any(|response| response.answer.as_ref() == "Yes"
                    && response.value == 10
                    && response.unit == Unit::Count
                    && matches!(response.demographic, Demographic::All))
        );
        assert!(
            questions[1]
                .responses
                .iter()
                .any(|response| response.answer.as_ref() == "Approve"
                    && response.value == 600
                    && response.unit == Unit::Other("percent_tenths".to_string()))
        );
    }
}
