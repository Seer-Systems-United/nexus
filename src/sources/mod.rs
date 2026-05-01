use std::error::Error;
use std::fmt;

pub(crate) mod date;
pub mod emerson;
pub mod gallup;
pub mod ipsos;
pub mod persistance;
pub mod yougov;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataCollection {
    pub title: String,
    pub subtitle: Option<String>,
    pub data: Vec<DataStructure>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataPanel {
    pub columns: Vec<String>,
    pub groups: Vec<DataGroup>,
    pub rows: Vec<DataRow>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataGroup {
    pub title: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataSeries {
    pub label: String,
    pub values: Vec<f32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataSlice {
    pub label: String,
    pub value: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DataRow {
    pub label: String,
    pub values: Vec<f32>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub enum Scope {
    // Get the latest polling data available, regardless of when it was collected.
    Latest,
    // Get the last N entries of polling data, regardless of when they were collected.
    LastNEntries(u32),
    // Get data collected in the last N days/weeks/months/years.
    LastDays(u32),
    LastWeeks(u32),
    LastMonths(u32),
    LastYears(u32),
}

impl Default for Scope {
    fn default() -> Self {
        Self::Latest
    }
}

impl Scope {
    pub(crate) fn cache_key(self) -> String {
        match self {
            Self::Latest => "latest".to_string(),
            Self::LastNEntries(count) => format!("last-{count}-entries"),
            Self::LastDays(count) => format!("last-{count}-days"),
            Self::LastWeeks(count) => format!("last-{count}-weeks"),
            Self::LastMonths(count) => format!("last-{count}-months"),
            Self::LastYears(count) => format!("last-{count}-years"),
        }
    }

    pub(crate) fn collection_label(self) -> String {
        match self {
            Self::Latest => "Latest".to_string(),
            Self::LastNEntries(count) => format!("Last {count} entries"),
            Self::LastDays(count) => format!("Last {count} days"),
            Self::LastWeeks(count) => format!("Last {count} weeks"),
            Self::LastMonths(count) => format!("Last {count} months"),
            Self::LastYears(count) => format!("Last {count} years"),
        }
    }

    pub(crate) fn entry_limit(self) -> Option<usize> {
        match self {
            Self::Latest => Some(1),
            Self::LastNEntries(count) => Some(count as usize),
            Self::LastDays(_) | Self::LastWeeks(_) | Self::LastMonths(_) | Self::LastYears(_) => {
                None
            }
        }
    }

    pub(crate) fn cutoff_date(
        self,
    ) -> Result<Option<date::SimpleDate>, Box<dyn Error + Send + Sync>> {
        let today = date::SimpleDate::today_utc()?;
        let cutoff = match self {
            Self::Latest | Self::LastNEntries(_) => return Ok(None),
            Self::LastDays(count) => today.days_prior(count)?,
            Self::LastWeeks(count) => today.days_prior(count.saturating_mul(7))?,
            Self::LastMonths(count) => today.months_prior(count),
            Self::LastYears(count) => today.years_prior(count),
        };

        Ok(Some(cutoff))
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.cache_key())
    }
}

#[async_trait::async_trait]
pub trait Source {
    const NAME: &'static str;
    const CACHE_VERSION: &'static str = "v1";

    async fn get_data(scope: Scope) -> Result<DataCollection, Box<dyn Error + Send + Sync>>;
}
