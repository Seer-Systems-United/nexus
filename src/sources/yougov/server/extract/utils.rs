pub(crate) const DOCUMENT_TITLE: &str = "The Economist/YouGov Poll";

pub(crate) fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn is_page_number(line: &str) -> bool {
    !line.is_empty() && line.chars().all(|ch| ch.is_ascii_digit())
}

pub(crate) fn is_question_title(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    if !matches!(chars.peek(), Some(ch) if ch.is_ascii_digit()) {
        return false;
    }
    while matches!(chars.peek(), Some(ch) if ch.is_ascii_digit()) {
        chars.next();
    }
    if matches!(chars.peek(), Some(ch) if ch.is_ascii_uppercase()) {
        chars.next();
    }
    matches!(chars.next(), Some('.')) && matches!(chars.next(), Some(' '))
}

pub(crate) fn is_table_of_contents_entry(line: &str) -> bool {
    line.contains(". .")
}

pub(crate) fn parse_percentage(token: &str) -> Option<f32> {
    token.strip_suffix('%')?.replace(',', "").parse().ok()
}

pub(crate) fn parse_row(line: &str, expected_values: usize) -> Option<crate::sources::DataRow> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }
    let mut value_count = 0usize;
    for token in tokens.iter().rev() {
        if parse_percentage(token).is_some() {
            value_count += 1;
        } else {
            break;
        }
    }
    if value_count != expected_values || value_count >= tokens.len() {
        return None;
    }
    let split_at = tokens.len() - value_count;
    let label = tokens[..split_at].join(" ");
    let values = tokens[split_at..]
        .iter()
        .filter_map(|token| parse_percentage(token))
        .collect();
    Some(crate::sources::DataRow { label, values })
}
