use std::error::Error;
use std::fmt;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub enum Scope {
    Latest,
    LastNEntries(u32),
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
    ) -> Result<Option<crate::sources::date::SimpleDate>, Box<dyn Error + Send + Sync>> {
        let today = crate::sources::date::SimpleDate::today_utc()?;
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
