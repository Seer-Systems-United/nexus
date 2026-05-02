use crate::sources::{DataCollection, Scope, Source, persistance::StorageWrapper};
use std::io::{Error as IoError, ErrorKind};

pub mod download;
pub mod extract;

#[derive(Debug, Clone)]
pub(crate) struct IpsosPollPdf {
    pub title: String,
    pub published_on: String,
    pub article_url: String,
    pub pdf_url: String,
    pub bytes: Vec<u8>,
}

async fn load_ipsos(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::Ipsos>::new();

    storage
        .get_data_with_cache(scope, |cached| async move {
            let pdfs = download::download_ipsos_polls(scope).await?;

            if pdfs.is_empty() {
                return cached.map(|snapshot| snapshot.data).ok_or_else(|| {
                    IoError::new(
                        ErrorKind::NotFound,
                        format!("Ipsos poll PDFs not found for scope {scope}"),
                    )
                    .into()
                });
            }

            extract::extract_ipsos_data(&pdfs, scope)
        })
        .await
}

#[async_trait::async_trait]
impl Source for super::Ipsos {
    const NAME: &'static str = "Ipsos";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_ipsos(scope).await
    }
}
