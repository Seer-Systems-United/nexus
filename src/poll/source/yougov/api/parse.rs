use std::{borrow::Cow, sync::Arc};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::poll::{
    question::{Question, is_non_question_text},
    response::{
        Response,
        demographic::{
            Demographic, education_level::EducationLevel, ethnicity::Ethnicity, ideology::Ideology,
            partisan_affiliation::PartisanAffiliation, sex::Sex,
        },
        unit::Unit,
    },
};

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

pub fn parse_pages(pages: &[String]) -> Vec<Question> {
    let mut parsed_questions: Vec<(usize, Question)> = pages
        .par_iter()
        .enumerate()
        .filter_map(|(index, page)| parse_page(page).map(|question| (index, question)))
        .collect();
    parsed_questions.sort_by_key(|(index, _)| *index);

    let mut questions: Vec<Question> = Vec::with_capacity(parsed_questions.len());

    for (_, question) in parsed_questions {
        if let Some(existing) = questions
            .iter_mut()
            .find(|existing| existing.text == question.text)
        {
            existing.responses.extend(question.responses);
        } else {
            questions.push(question);
        }
    }

    questions
}

fn build_question_text(question_lines: &[&str]) -> Option<String> {
    let mut question_lines: Vec<&str> = question_lines
        .iter()
        .copied()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    while question_lines
        .last()
        .is_some_and(|line| is_question_artifact_line(line))
    {
        question_lines.pop();
    }

    if question_lines.len() > 1
        && is_question_title_line(question_lines[0])
        && !question_lines[0].contains('?')
    {
        question_lines.remove(0);
    }

    let question = clean_question_text(&question_lines.join(" "));

    if question.is_empty() || is_incomplete_question_stem(&question) {
        None
    } else {
        Some(question)
    }
}

fn clean_question_text(question: &str) -> String {
    let mut question = question.split_whitespace().collect::<Vec<_>>().join(" ");
    question = strip_question_title_prefix(&question).to_string();

    loop {
        let mut changed = false;

        for suffix in [
            "Sex Race Age Education",
            "Sex Race Age",
            "Sex Race",
            "2024 Vote Reg Ideology MAGA Party ID",
            "2024 Vote Reg Ideology",
            "MAGA Party ID",
            "Party ID",
            "that apply",
        ] {
            if let Some(prefix) = question.strip_suffix(suffix) {
                let prefix = prefix.trim_end();

                if !prefix.is_empty() && should_strip_question_suffix(prefix, suffix) {
                    question = prefix.to_string();
                    changed = true;
                    break;
                }
            }
        }

        if !changed {
            break;
        }
    }

    question
}

fn strip_question_title_prefix(question: &str) -> &str {
    let Some((prefix, rest)) = question.split_once(' ') else {
        return question;
    };

    if is_question_title_line(prefix) {
        rest
    } else {
        question
    }
}

fn should_strip_question_suffix(prefix: &str, suffix: &str) -> bool {
    suffix != "that apply" || prefix.contains("check all that apply.")
}

fn is_question_artifact_line(line: &str) -> bool {
    matches!(
        line,
        "Sex Race"
            | "Age"
            | "Education"
            | "Sex Race Age Education"
            | "Sex Race Age"
            | "2024 Vote Reg Ideology"
            | "MAGA"
            | "Party"
            | "ID"
            | "2024 Vote Reg Ideology MAGA Party ID"
            | "that apply"
    )
}

fn is_question_title_line(line: &str) -> bool {
    let mut chars = line.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_ascii_digit() {
        return false;
    }

    for c in chars {
        if c == '.' {
            return true;
        }

        if !c.is_ascii_digit() && !c.is_ascii_uppercase() {
            return false;
        }
    }

    false
}

fn is_incomplete_question_stem(question: &str) -> bool {
    question.ends_with("...")
}

fn parse_responses_from_iter<'a, I>(first_header: &'a str, lines: &mut I) -> Vec<Response>
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

fn parse_column_specs(line: &str) -> Option<Vec<ColumnSpec>> {
    let mut tokens = line.split_whitespace().peekable();
    if tokens.next()? != "Total" {
        return None;
    }

    let mut labels = Vec::with_capacity(12);
    labels.push(ColumnSpec::Total);

    while let Some(token) = tokens.next() {
        if token == "No" && tokens.peek().copied() == Some("degree") {
            labels.push(ColumnSpec::NoDegree);
            tokens.next();
        } else if token == "College" && tokens.peek().copied() == Some("grad") {
            labels.push(ColumnSpec::CollegeGrad);
            tokens.next();
        } else {
            labels.push(ColumnSpec::from_label(token));
        }
    }

    if matches!(
        labels.as_slice(),
        [ColumnSpec::Total, ColumnSpec::Harris, ColumnSpec::Trump]
    ) {
        return Some(vec![
            ColumnSpec::Total,
            ColumnSpec::Harris,
            ColumnSpec::Trump,
            ColumnSpec::Voters,
            ColumnSpec::Lib,
            ColumnSpec::Mod,
            ColumnSpec::Con,
            ColumnSpec::Supporter,
            ColumnSpec::Dem,
            ColumnSpec::Ind,
            ColumnSpec::Rep,
        ]);
    }

    Some(labels)
}

fn split_response_line(line: &str) -> (Option<&str>, Vec<(u16, Unit)>) {
    let mut first_cell_start = None;
    let mut values = Vec::with_capacity(16);
    let mut offset = 0;

    for token in line.split_whitespace() {
        let start = offset
            + line[offset..]
                .find(token)
                .expect("split_whitespace token exists in line");
        offset = start + token.len();

        if let Some(cell) = parse_cell(token) {
            first_cell_start.get_or_insert(start);
            values.push(cell);
        }
    }

    let answer = first_cell_start
        .map(|start| line[..start].trim())
        .filter(|answer| !answer.is_empty())
        .or_else(|| {
            if values.is_empty() {
                Some(line.trim()).filter(|answer| !answer.is_empty())
            } else {
                None
            }
        });

    (answer, values)
}

fn parse_cell(token: &str) -> Option<(u16, Unit)> {
    if let Some(value) = token.strip_suffix('%') {
        if value.is_empty() {
            return None;
        }

        return Some((parse_u16(value)?, Unit::Percent));
    }

    None
}

fn parse_u16(value: &str) -> Option<u16> {
    let mut parsed = 0u16;

    for byte in value.bytes() {
        if !byte.is_ascii_digit() {
            return None;
        }

        parsed = parsed
            .checked_mul(10)?
            .checked_add(u16::from(byte - b'0'))?;
    }

    Some(parsed)
}

fn demographic_for_column(column: &ColumnSpec) -> Demographic {
    match column {
        ColumnSpec::Total => Demographic::All,
        ColumnSpec::Male => Demographic::Sex { sex: Sex::Male },
        ColumnSpec::Female => Demographic::Sex { sex: Sex::Female },
        ColumnSpec::White => Demographic::Ethnicity {
            ethnicity: Ethnicity::White,
        },
        ColumnSpec::Black => Demographic::Ethnicity {
            ethnicity: Ethnicity::Black,
        },
        ColumnSpec::Asian => Demographic::Ethnicity {
            ethnicity: Ethnicity::Asian,
        },
        ColumnSpec::Hispanic => Demographic::Ethnicity {
            ethnicity: Ethnicity::Hispanic,
        },
        ColumnSpec::Age18To29 => Demographic::Age {
            lower_bound: 18,
            upper_bound: 29,
        },
        ColumnSpec::Age30To44 => Demographic::Age {
            lower_bound: 30,
            upper_bound: 44,
        },
        ColumnSpec::Age45To64 => Demographic::Age {
            lower_bound: 45,
            upper_bound: 64,
        },
        ColumnSpec::Age65Plus => Demographic::Age {
            lower_bound: 65,
            upper_bound: u8::MAX,
        },
        ColumnSpec::NoDegree => Demographic::EducationLevel {
            education_level: EducationLevel::NoDegree,
        },
        ColumnSpec::CollegeGrad => Demographic::EducationLevel {
            education_level: EducationLevel::CollegeGrad,
        },
        ColumnSpec::Voters => Demographic::VoterRegistration { regeristered: true },
        ColumnSpec::Lib => Demographic::Ideology {
            ideology: Ideology::Liberal,
        },
        ColumnSpec::Con => Demographic::Ideology {
            ideology: Ideology::Conservative,
        },
        ColumnSpec::Dem => Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Democrat,
        },
        ColumnSpec::Ind => Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Independent,
        },
        ColumnSpec::Mod => Demographic::Ideology {
            ideology: Ideology::Moderate,
        },
        ColumnSpec::Rep => Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Republican,
        },
        ColumnSpec::Harris => Demographic::Other {
            description: Cow::Borrowed("2024 vote: Harris"),
        },
        ColumnSpec::Trump => Demographic::Other {
            description: Cow::Borrowed("2024 vote: Trump"),
        },
        ColumnSpec::Supporter => Demographic::Other {
            description: Cow::Borrowed("MAGA supporter"),
        },
        ColumnSpec::Other(column) => Demographic::Other {
            description: Cow::Owned(column.clone()),
        },
    }
}

fn is_column_header_line(line: &str) -> bool {
    line.starts_with("Total ")
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

enum ColumnSpec {
    Total,
    Male,
    Female,
    White,
    Black,
    Asian,
    Hispanic,
    Age18To29,
    Age30To44,
    Age45To64,
    Age65Plus,
    NoDegree,
    CollegeGrad,
    Voters,
    Lib,
    Mod,
    Con,
    Dem,
    Ind,
    Rep,
    Harris,
    Trump,
    Supporter,
    Other(String),
}

impl ColumnSpec {
    fn from_label(label: &str) -> Self {
        match label {
            "Total" => Self::Total,
            "Male" => Self::Male,
            "Female" => Self::Female,
            "White" => Self::White,
            "Black" => Self::Black,
            "Asian" => Self::Asian,
            "Hispanic" => Self::Hispanic,
            "18-29" => Self::Age18To29,
            "30-44" => Self::Age30To44,
            "45-64" => Self::Age45To64,
            "65+" => Self::Age65Plus,
            "Voters" => Self::Voters,
            "Lib" => Self::Lib,
            "Mod" => Self::Mod,
            "Con" => Self::Con,
            "Dem" => Self::Dem,
            "Ind" => Self::Ind,
            "Rep" => Self::Rep,
            "Harris" => Self::Harris,
            "Trump" => Self::Trump,
            "Supporter" => Self::Supporter,
            _ => Self::Other(label.to_string()),
        }
    }
}
