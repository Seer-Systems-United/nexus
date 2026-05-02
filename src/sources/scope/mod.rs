//! # Query scope types for polling sources
//!
//! Defines the `Scope` enum for specifying how much polling data
//! to load: latest, last N entries, or time-based windows.

use std::error::Error;
use std::fmt;

/// Scope for loading polling source data.
///
/// # Variants
/// - `Latest`: Only the most recent poll.
/// - `LastNEntries(n)`: Last n poll entries.
/// - `LastDays(n)`: Polls from the last n days.
/// - `LastWeeks(n)`: Polls from the last n weeks.
/// - `LastMonths(n)`: Polls from the last n months.
/// - `LastYears(n)`: Polls from the last n years.
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
    /// Generate a cache key string for this scope.
    ///
    /// # Returns
    /// - A string like "latest", "last-5-entries", "last-30-days", etc.
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

    /// Generate a human-readable label for this scope.
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

    /// Get the maximum number of entries to return (if applicable).
    ///
    /// # Returns
    /// - `Some(n)` for `LastNEntries`.
    /// - `None` for other scopes (unlimited by entry count).
    pub(crate) fn entry_limit(self) -> Option<usize> {
        match self {
            Self::Latest => Some(1),
            Self::LastNEntries(count) => Some(count as usize),
            Self::LastDays(_) | Self::LastWeeks(_) | Self::LastMonths(_) | Self::LastYears(_) => {
                None
            }
        }
    }

    /// Calculate the cutoff date for this scope based on today's date.
    ///
    /// # Returns
    /// - `Ok(Some(SimpleDate))`: The cutoff date.
    /// - `Ok(None)`: No cutoff needed (Latest or LastNEntries).
    ///
    /// # Errors
    /// - Returns an error if date calculation fails.
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
