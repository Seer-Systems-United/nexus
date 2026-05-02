use crate::sources::DataGroup;
use crate::sources::emerson::server::extract::utils::cell_text;

pub(super) struct CrosstabLayout {
    pub(super) columns: Vec<String>,
    pub(super) groups: Vec<DataGroup>,
    pub(super) row_indices: Vec<usize>,
}

pub(super) fn parse_crosstab_layout(rows: &[Vec<calamine::Data>]) -> Option<CrosstabLayout> {
    let mut layout = LayoutBuilder::default();

    for row_index in 3..rows.len() {
        if !layout.add_row(rows, row_index) {
            break;
        }
    }

    (!layout.columns.is_empty()).then(|| layout.finish())
}

#[derive(Default)]
struct LayoutBuilder {
    columns: Vec<String>,
    groups: Vec<DataGroup>,
    row_indices: Vec<usize>,
    current_group_index: Option<usize>,
    empty_streak: usize,
}

impl LayoutBuilder {
    fn add_row(&mut self, rows: &[Vec<calamine::Data>], row_index: usize) -> bool {
        let group_title = cell_text(rows, row_index, 0);
        let subgroup_label = cell_text(rows, row_index, 1);

        if group_title.is_empty() && subgroup_label.is_empty() {
            return self.record_empty_row();
        }

        self.empty_streak = 0;
        if !group_title.is_empty() {
            self.push_group(group_title);
        }
        if !subgroup_label.is_empty() {
            self.push_column(subgroup_label, row_index);
        }
        true
    }

    fn record_empty_row(&mut self) -> bool {
        if !self.columns.is_empty() {
            self.empty_streak += 1;
        }
        self.empty_streak < 3
    }

    fn push_group(&mut self, title: String) {
        self.groups.push(DataGroup {
            title,
            labels: Vec::new(),
        });
        self.current_group_index = Some(self.groups.len() - 1);
    }

    fn push_column(&mut self, label: String, row_index: usize) {
        let group_index = self.current_group_index.unwrap_or_else(|| {
            self.push_group("Overall".to_string());
            self.groups.len() - 1
        });
        self.groups[group_index].labels.push(label.clone());
        self.columns.push(label);
        self.row_indices.push(row_index);
    }

    fn finish(self) -> CrosstabLayout {
        CrosstabLayout {
            columns: self.columns,
            groups: self.groups,
            row_indices: self.row_indices,
        }
    }
}
