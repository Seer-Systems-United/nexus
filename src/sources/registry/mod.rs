//! # Polling source registry
//!
//! Defines the `SourceId` enum for identifying polling sources
//! and dispatching data loading to the appropriate source implementation.

use crate::sources::{DataCollection, Scope, Source};

/// Identifier for a polling source.
///
/// # Variants
/// - `Emerson`: Emerson College Polling.
/// - `Gallup`: Gallup Poll.
/// - `Ipsos`: Ipsos Poll.
/// - `YouGov`: YouGov Poll.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum SourceId {
    Emerson,
    Gallup,
    Ipsos,
    YouGov,
}

impl SourceId {
    /// All available source IDs.
    pub const ALL: [Self; 4] = [Self::Emerson, Self::Gallup, Self::Ipsos, Self::YouGov];

    /// Parse a source ID from a string (case-insensitive).
    ///
    /// # Parameters
    /// - `input`: The string to parse ("emerson", "gallup", "ipsos", "yougov").
    ///
    /// # Returns
    /// - `Some(SourceId)` if recognized, `None` otherwise.
    pub fn parse(input: &str) -> Option<Self> {
        match input.trim().to_ascii_lowercase().as_str() {
            "emerson" => Some(Self::Emerson),
            "gallup" => Some(Self::Gallup),
            "ipsos" => Some(Self::Ipsos),
            "yougov" | "you-gov" => Some(Self::YouGov),
            _ => None,
        }
    }

    /// Get the static string ID for this source.
    ///
    /// # Returns
    /// - "emerson", "gallup", "ipsos", or "yougov".
    pub fn id(self) -> &'static str {
        match self {
            Self::Emerson => "emerson",
            Self::Gallup => "gallup",
            Self::Ipsos => "ipsos",
            Self::YouGov => "yougov",
        }
    }

    /// Get the display name for this source.
    pub fn name(self) -> &'static str {
        match self {
            Self::Emerson => "Emerson",
            Self::Gallup => "Gallup",
            Self::Ipsos => "Ipsos",
            Self::YouGov => "YouGov",
        }
    }

    /// Load data from this source for the given scope.
    ///
    /// # Parameters
    /// - `scope`: The scope defining how much data to load.
    ///
    /// # Returns
    /// - `Ok(DataCollection)`: The loaded data.
    ///
    /// # Errors
    /// - Returns an error if data loading fails.
    pub async fn load(
        self,
        scope: Scope,
    ) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::Emerson => crate::sources::emerson::Emerson::get_data(scope).await,
            Self::Gallup => crate::sources::gallup::Gallup::get_data(scope).await,
            Self::Ipsos => crate::sources::ipsos::Ipsos::get_data(scope).await,
            Self::YouGov => crate::sources::yougov::YouGov::get_data(scope).await,
        }
    }
}
