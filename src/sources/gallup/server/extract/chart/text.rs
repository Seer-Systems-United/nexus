pub(super) fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(super) fn parse_number(value: &str) -> Option<f32> {
    let normalized = value
        .trim()
        .trim_end_matches('%')
        .replace(',', "")
        .replace('<', "");

    if normalized.is_empty() || normalized == "-" || normalized.eq_ignore_ascii_case("n/a") {
        return None;
    }

    normalized.parse().ok()
}

pub(super) fn looks_temporal(label: &str) -> bool {
    let normalized = label.trim();

    normalized.len() == 4 && normalized.chars().all(|ch| ch.is_ascii_digit())
        || normalized.contains('/')
        || normalized.contains('-')
}
