const COMMON_NOISE_LINES: &[&str] = &[
    "TOPLINE & METHODOLOGY",
    "2020 K Street, NW, Suite 410",
    "Washington DC 20006",
    "+1 202 463-7300",
    "Contact:",
    "Email:",
    "Annotated Questionnaire",
];

pub fn normalize_line(line: &str) -> String {
    let cleaned = line
        .replace("TOPLINE & METHODOLOGY", "")
        .replace("Annotated Questionnaire", "");

    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(super) fn is_noise_line(line: &str) -> bool {
    line.is_empty()
        || line.chars().all(|ch| ch.is_ascii_digit())
        || COMMON_NOISE_LINES
            .iter()
            .any(|noise| line.eq_ignore_ascii_case(noise))
}

pub(super) fn parse_value_token(token: &str) -> Option<f32> {
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

pub(super) fn sample_size_index(line: &str) -> Option<usize> {
    line.find("(N=").or_else(|| line.find("N="))
}

pub(super) fn sample_size_end(line: &str, sample_index: usize) -> usize {
    line[sample_index..]
        .find(')')
        .map(|offset| sample_index + offset + 1)
        .unwrap_or(line.len())
}

pub(super) fn is_sample_size_line(line: &str) -> bool {
    sample_size_index(line).is_some()
}

pub(super) fn next_label_has_sample_size(lines: &[String], start: usize) -> bool {
    for line in lines.iter().skip(start).take(8) {
        if is_noise_line(line) {
            continue;
        }
        if is_sample_size_line(line) {
            return true;
        }
        let last_token = line.split_whitespace().last().unwrap_or_default();
        if parse_value_token(last_token).is_some() || is_question_title(line) {
            return false;
        }
    }

    false
}

pub fn is_question_title(line: &str) -> bool {
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
