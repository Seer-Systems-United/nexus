//! # Source data types
//!
//! Defines common data structures for polling source responses,
//! including bar graphs, line graphs, pie charts, and crosstabs.
//! These types are serialized/deserialized with serde and documented in OpenAPI.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
/// A single data structure representing polling results.
///
/// # Variants
/// - `Unstructured`: Raw text data.
/// - `BarGraph`: Bar chart with title, labels, and values.
/// - `LineGraph`: Line chart with multiple series over x-axis labels.
/// - `PieChart`: Pie chart with labeled slices.
/// - `Crosstab`: Cross-tabulation with panels of demographic groups.
pub enum DataStructure {
    Unstructured {
        data: String,
    },
    BarGraph {
        title: String,
        x: Vec<String>,
        y: Vec<f32>,
        y_unit: String,
    },
    LineGraph {
        title: String,
        x: Vec<String>,
        series: Vec<DataSeries>,
        y_unit: String,
    },
    PieChart {
        title: String,
        slices: Vec<DataSlice>,
        y_unit: String,
    },
    Crosstab {
        title: String,
        prompt: String,
        panels: Vec<DataPanel>,
        y_unit: String,
    },
}

/// A collection of data structures with a title and optional subtitle.
///
/// # Fields
/// - `title`: The main title for the collection.
/// - `subtitle`: Optional subtitle (e.g., source and date info).
/// - `data`: The list of data structures in this collection.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataCollection {
    pub title: String,
    pub subtitle: Option<String>,
    pub data: Vec<DataStructure>,
}

/// A panel in a crosstab containing columns, groups, and rows.
///
/// # Fields
/// - `columns`: Column headers for the crosstab.
/// - `groups`: Grouped row labels (e.g., demographic breakdowns).
/// - `rows`: Data rows with labels and values.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataPanel {
    pub columns: Vec<String>,
    pub groups: Vec<DataGroup>,
    pub rows: Vec<DataRow>,
}

/// A group of labels within a crosstab panel.
///
/// # Fields
/// - `title`: The group title (e.g., "Overall", "By Party").
/// - `labels`: Sub-labels within this group.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataGroup {
    pub title: String,
    pub labels: Vec<String>,
}

/// A series of data points for a line graph.
///
/// # Fields
/// - `label`: The series label (e.g., "Approval", "Disapproval").
/// - `values`: The data values for each x-axis point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataSeries {
    pub label: String,
    pub values: Vec<f32>,
}

/// A single slice in a pie chart.
///
/// # Fields
/// - `label`: The slice label.
/// - `value`: The percentage value for this slice.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataSlice {
    pub label: String,
    pub value: f32,
}

/// A row in a crosstab data structure.
///
/// # Fields
/// - `label`: The row label (e.g., demographic category).
/// - `values`: The percentage values for each column.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataRow {
    pub label: String,
    pub values: Vec<f32>,
}
