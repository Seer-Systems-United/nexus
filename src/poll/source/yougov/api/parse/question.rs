use crate::poll::question::is_non_question_text;

pub(super) fn build_question_text(question_lines: &[&str]) -> Option<String> {
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

    if question.is_empty()
        || is_incomplete_question_stem(&question)
        || is_non_question_text(&question)
    {
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
