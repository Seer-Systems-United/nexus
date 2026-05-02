use crate::sources::{DataCollection, Scope, Source};

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
    pub const ALL: [Self; 4] = [Self::Emerson, Self::Gallup, Self::Ipsos, Self::YouGov];

    pub fn parse(input: &str) -> Option<Self> {
        match input.trim().to_ascii_lowercase().as_str() {
            "emerson" => Some(Self::Emerson),
            "gallup" => Some(Self::Gallup),
            "ipsos" => Some(Self::Ipsos),
            "yougov" | "you-gov" => Some(Self::YouGov),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Emerson => "emerson",
            Self::Gallup => "gallup",
            Self::Ipsos => "ipsos",
            Self::YouGov => "yougov",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Emerson => "Emerson",
            Self::Gallup => "Gallup",
            Self::Ipsos => "Ipsos",
            Self::YouGov => "YouGov",
        }
    }

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
