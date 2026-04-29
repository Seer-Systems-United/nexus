use crate::sources::{DataCollection, Scope, Source, persistance::StorageWrapper};
use std::io::{Error as IoError, ErrorKind};

pub mod download;
pub mod extract;

#[derive(Debug, Clone)]
pub(crate) struct GallupChartAsset {
    pub title: String,
    pub csv_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct GallupArticleAsset {
    pub title: String,
    pub published_on: String,
    pub pdf_bytes: Option<Vec<u8>>,
    pub charts: Vec<GallupChartAsset>,
}

async fn load_gallup(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::Gallup>::new();

    storage
        .get_data_with_cache(scope, |cached| async move {
            let articles = download::download_gallup_articles(scope).await?;

            if articles.is_empty() {
                return cached.map(|snapshot| snapshot.data).ok_or_else(|| {
                    IoError::new(
                        ErrorKind::NotFound,
                        format!("Gallup articles not found for scope {scope}"),
                    )
                    .into()
                });
            }

            extract::extract_gallup_data(&articles, scope)
        })
        .await
}

#[async_trait::async_trait]
impl Source for super::Gallup {
    const NAME: &'static str = "Gallup";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_gallup(scope).await
    }
}
