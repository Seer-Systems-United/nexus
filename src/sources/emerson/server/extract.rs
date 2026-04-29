use crate::sources::{DataCollection, DataGroup, DataPanel, DataRow, DataStructure, Scope};
use calamine::Reader;
use std::error::Error;

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone)]
struct CrosstabLayout {
    columns: Vec<String>,
    groups: Vec<DataGroup>,
    row_indices: Vec<usize>,
}

const TOPLINE_SHEET_NAME: &str = "Topline Results";
const CROSSTABS_SHEET_NAME: &str = "crosstabs";
const FULL_CROSSTABS_SHEET_NAME: &str = "full crosstabs";
const SELECTED_CHOICE_SUFFIX: &str = " - Selected Choice";

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn cell_text(rows: &[Vec<calamine::Data>], row: usize, col: usize) -> String {
    rows.get(row)
        .and_then(|cells| cells.get(col))
        .map(cell_to_string)
        .unwrap_or_default()
}

fn cell_to_string(cell: &calamine::Data) -> String {
    match cell {
        calamine::Data::Empty => String::new(),
        _ => normalize_text(&cell.to_string()),
    }
}

fn cell_number(rows: &[Vec<calamine::Data>], row: usize, col: usize) -> Option<f64> {
    let cell = rows.get(row)?.get(col)?;
    match cell {
        calamine::Data::Float(value) => Some(*value),
        calamine::Data::Int(value) => Some(*value as f64),
        _ => cell.to_string().trim().parse().ok(),
    }
}

fn row_is_empty(row: &[calamine::Data]) -> bool {
    row.iter().all(|cell| cell_to_string(cell).is_empty())
}

fn is_total_label(label: &str) -> bool {
    label.eq_ignore_ascii_case("total")
}

fn scale_percent(value: f64) -> f32 {
    if value.abs() <= 1.5 {
        (value * 100.0) as f32
    } else {
        value as f32
    }
}

fn clean_question_title(title: &str) -> String {
    let normalized = normalize_text(title);
    normalized
        .strip_suffix(SELECTED_CHOICE_SUFFIX)
        .unwrap_or(&normalized)
        .trim()
        .to_string()
}

// === Topline Parsing ===

fn parse_topline_sheet(rows: &[Vec<calamine::Data>]) -> Vec<DataStructure> {
    let mut charts = Vec::new();
    let mut index = 0usize;

    while index < rows.len() {
        let question = cell_text(rows, index, 0);
        if question.is_empty() {
            index += 1;
            continue;
        }

        let mut header_row = index + 1;
        while header_row < rows.len() && row_is_empty(&rows[header_row]) {
            header_row += 1;
        }

        if header_row >= rows.len()
            || cell_text(rows, header_row, 2) != "Frequency"
            || cell_text(rows, header_row, 3) != "Valid Percent"
        {
            index += 1;
            continue;
        }

        let mut labels = Vec::new();
        let mut values = Vec::new();
        let mut cursor = header_row + 1;

        while cursor < rows.len() {
            if row_is_empty(&rows[cursor]) {
                if !labels.is_empty() {
                    break;
                }
                cursor += 1;
                continue;
            }

            if !cell_text(rows, cursor, 0).is_empty() {
                break;
            }

            let label = cell_text(rows, cursor, 1);
            let percent = cell_number(rows, cursor, 3);

            if !label.is_empty() && !is_total_label(&label) {
                if let Some(percent) = percent {
                    labels.push(label);
                    values.push(percent as f32);
                }
            }

            cursor += 1;
        }

        if !labels.is_empty() {
            charts.push(DataStructure::BarGraph {
                title: question,
                x: labels,
                y: values,
                y_unit: "%".to_string(),
            });
            index = cursor;
            continue;
        }

        index += 1;
    }

    charts
}

// === Crosstab Parsing ===

fn parse_crosstab_layout(rows: &[Vec<calamine::Data>]) -> Option<CrosstabLayout> {
    let mut columns = Vec::new();
    let mut groups = Vec::new();
    let mut row_indices = Vec::new();
    let mut current_group_index = None;
    let mut empty_streak = 0usize;

    for row_index in 3..rows.len() {
        let group_title = cell_text(rows, row_index, 0);
        let subgroup_label = cell_text(rows, row_index, 1);

        if group_title.is_empty() && subgroup_label.is_empty() {
            if !columns.is_empty() {
                empty_streak += 1;
                if empty_streak >= 3 {
                    break;
                }
            }
            continue;
        }

        empty_streak = 0;

        if !group_title.is_empty() {
            groups.push(DataGroup {
                title: group_title,
                labels: Vec::new(),
            });
            current_group_index = Some(groups.len() - 1);
        }

        if subgroup_label.is_empty() {
            continue;
        }

        let group_index = match current_group_index {
            Some(index) => index,
            None => {
                groups.push(DataGroup {
                    title: "Overall".to_string(),
                    labels: Vec::new(),
                });
                let index = groups.len() - 1;
                current_group_index = Some(index);
                index
            }
        };

        groups[group_index].labels.push(subgroup_label.clone());
        columns.push(subgroup_label);
        row_indices.push(row_index);
    }

    (!columns.is_empty()).then_some(CrosstabLayout {
        columns,
        groups,
        row_indices,
    })
}

fn parse_crosstab_row(
    rows: &[Vec<calamine::Data>],
    label: String,
    row_indices: &[usize],
    percent_col: usize,
) -> Option<DataRow> {
    if label.is_empty() || is_total_label(&label) {
        return None;
    }

    let values = row_indices
        .iter()
        .map(|row_index| {
            cell_number(rows, *row_index, percent_col)
                .map(scale_percent)
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    Some(DataRow { label, values })
}

fn parse_crosstab_sheet(rows: &[Vec<calamine::Data>]) -> Vec<DataStructure> {
    let Some(layout) = parse_crosstab_layout(rows) else {
        return Vec::new();
    };

    let max_cols = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut block_starts = Vec::new();

    for col in 2..max_cols {
        let title = cell_text(rows, 0, col);
        if !title.is_empty() {
            block_starts.push((col, clean_question_title(&title)));
        }
    }

    let mut structures = Vec::new();

    for (index, (start_col, block_title)) in block_starts.iter().enumerate() {
        let end_col = block_starts
            .get(index + 1)
            .map(|(next_start, _)| next_start.saturating_sub(1))
            .unwrap_or(max_cols.saturating_sub(1));

        let mut data_rows = Vec::new();
        let mut option_col = *start_col;

        while option_col <= end_col {
            let answer_label = cell_text(rows, 1, option_col);
            if answer_label.is_empty() {
                option_col += 1;
                continue;
            }

            let percent_col = option_col + 1;
            if percent_col > end_col {
                break;
            }

            if let Some(row) =
                parse_crosstab_row(rows, answer_label, &layout.row_indices, percent_col)
            {
                data_rows.push(row);
            }

            option_col += 2;
        }

        if data_rows.is_empty() {
            continue;
        }

        structures.push(DataStructure::Crosstab {
            title: block_title.clone(),
            prompt: block_title.clone(),
            panels: vec![DataPanel {
                columns: layout.columns.clone(),
                groups: layout.groups.clone(),
                rows: data_rows,
            }],
            y_unit: "%".to_string(),
        });
    }

    structures
}

// === Main Extract Function ===

pub fn extract_emerson_data(
    workbooks: &[crate::sources::emerson::server::EmersonWorkbook],
    _scope: Scope,
) -> Result<DataCollection, Box<dyn Error + Send + Sync>> {
    let mut data = Vec::new();

    for workbook in workbooks {
        let mut wb: calamine::Xlsx<_> =
            calamine::open_workbook_from_rs(std::io::Cursor::new(&workbook.bytes))?;

        if let Ok(range) = wb.worksheet_range(TOPLINE_SHEET_NAME) {
            let rows = range.rows().map(|row| row.to_vec()).collect::<Vec<_>>();
            data.extend(parse_topline_sheet(&rows));
        }

        for sheet_name in [CROSSTABS_SHEET_NAME, FULL_CROSSTABS_SHEET_NAME] {
            if let Ok(range) = wb.worksheet_range(sheet_name) {
                let rows = range.rows().map(|row| row.to_vec()).collect::<Vec<_>>();
                data.extend(parse_crosstab_sheet(&rows));
            }
        }
    }

    if data.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no Emerson poll data found in workbook",
        )));
    }

    let subtitle = workbooks
        .first()
        .and_then(|wb| Some(format!("Polling data: {}", wb.date)));
    Ok(DataCollection {
        title: "Emerson Polls".to_string(),
        subtitle,
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_crosstab_sheet;
    use crate::sources::DataStructure;

    fn empty() -> calamine::Data {
        calamine::Data::Empty
    }

    fn text(value: &str) -> calamine::Data {
        calamine::Data::String(value.to_string())
    }

    fn number(value: f64) -> calamine::Data {
        calamine::Data::Float(value)
    }

    #[test]
    fn crosstab_rows_are_answer_options_with_values_across_columns() {
        let rows = vec![
            vec![
                empty(),
                empty(),
                text("Do you approve? - Selected Choice"),
                empty(),
                empty(),
                empty(),
            ],
            vec![
                empty(),
                empty(),
                text("Approve"),
                empty(),
                text("Disapprove"),
                empty(),
            ],
            vec![],
            vec![
                text("Party"),
                text("Democrat"),
                empty(),
                number(0.75),
                empty(),
                number(0.15),
            ],
            vec![
                empty(),
                text("Republican"),
                empty(),
                number(0.20),
                empty(),
                number(0.70),
            ],
            vec![
                text("Race"),
                text("White"),
                empty(),
                number(45.0),
                empty(),
                number(40.0),
            ],
        ];

        let structures = parse_crosstab_sheet(&rows);

        assert_eq!(structures.len(), 1);
        let DataStructure::Crosstab { title, panels, .. } = &structures[0] else {
            panic!("expected crosstab");
        };

        assert_eq!(title, "Do you approve?");
        assert_eq!(panels[0].columns, vec!["Democrat", "Republican", "White"]);
        assert_eq!(panels[0].rows.len(), 2);
        assert_eq!(panels[0].rows[0].label, "Approve");
        assert_eq!(panels[0].rows[0].values, vec![75.0, 20.0, 45.0]);
        assert_eq!(panels[0].rows[1].label, "Disapprove");
        assert_eq!(panels[0].rows[1].values, vec![15.0, 70.0, 40.0]);
    }
}
