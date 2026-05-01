use super::utils::*;

pub(crate) fn parse_crosstab_sheet(
    rows: &[Vec<calamine::Data>],
) -> Vec<crate::sources::DataStructure> {
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

        structures.push(crate::sources::DataStructure::Crosstab {
            title: block_title.clone(),
            prompt: block_title.clone(),
            panels: vec![crate::sources::DataPanel {
                columns: layout.columns.clone(),
                groups: layout.groups.clone(),
                rows: data_rows,
            }],
            y_unit: "%".to_string(),
        });
    }

    structures
}

pub(crate) fn parse_crosstab_layout(rows: &[Vec<calamine::Data>]) -> Option<CrosstabLayout> {
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
            groups.push(crate::sources::DataGroup {
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
                groups.push(crate::sources::DataGroup {
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

pub(crate) struct CrosstabLayout {
    pub(crate) columns: Vec<String>,
    pub(crate) groups: Vec<crate::sources::DataGroup>,
    pub(crate) row_indices: Vec<usize>,
}

fn parse_crosstab_row(
    rows: &[Vec<calamine::Data>],
    label: String,
    row_indices: &[usize],
    percent_col: usize,
) -> Option<crate::sources::DataRow> {
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

    Some(crate::sources::DataRow { label, values })
}
