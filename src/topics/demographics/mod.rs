mod groups;
mod labels;

use crate::sources::DataPanel;
use crate::topics::types::DemographicValue;
use groups::group_for_column;
use labels::{demographic_for_label, normalize_text};

pub fn demographic_for_panel_column(panel: &DataPanel, column_index: usize) -> DemographicValue {
    let column = panel
        .columns
        .get(column_index)
        .map(|column| normalize_text(column))
        .unwrap_or_else(|| "Total".to_string());
    let group = group_for_column(panel, &column);
    demographic_for_label(&column, group)
}

pub fn total_demographic() -> DemographicValue {
    demographic_for_label("Total", None)
}
