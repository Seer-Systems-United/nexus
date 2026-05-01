pub(crate) const TOPLINE_SHEET_NAME: &str = "Topline Results";
pub(crate) const CROSSTABS_SHEET_NAME: &str = "crosstabs";
pub(crate) const FULL_CROSSTABS_SHEET_NAME: &str = "full crosstabs";
const SELECTED_CHOICE_SUFFIX: &str = " - Selected Choice";

pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn cell_text(rows: &[Vec<calamine::Data>], row: usize, col: usize) -> String {
    rows.get(row)
        .and_then(|cells| cells.get(col))
        .map(cell_to_string)
        .unwrap_or_default()
}

pub(crate) fn cell_to_string(cell: &calamine::Data) -> String {
    match cell {
        calamine::Data::Empty => String::new(),
        _ => normalize_text(&cell.to_string()),
    }
}

pub(crate) fn cell_number(rows: &[Vec<calamine::Data>], row: usize, col: usize) -> Option<f64> {
    let cell = rows.get(row)?.get(col)?;
    match cell {
        calamine::Data::Float(value) => Some(*value),
        calamine::Data::Int(value) => Some(*value as f64),
        _ => cell.to_string().trim().parse().ok(),
    }
}

pub(crate) fn row_is_empty(row: &[calamine::Data]) -> bool {
    row.iter().all(|cell| cell_to_string(cell).is_empty())
}

pub(crate) fn is_total_label(label: &str) -> bool {
    label.eq_ignore_ascii_case("total")
}

pub(crate) fn scale_percent(value: f64) -> f32 {
    if value.abs() <= 1.5 {
        (value * 100.0) as f32
    } else {
        value as f32
    }
}

pub(crate) fn clean_question_title(title: &str) -> String {
    let normalized = normalize_text(title);
    normalized
        .strip_suffix(SELECTED_CHOICE_SUFFIX)
        .unwrap_or(&normalized)
        .trim()
        .to_string()
}
