use crate::sources::{DataCollection, Scope, persistance::StorageWrapper};

pub mod download;

pub mod extract;

async fn load_yougov(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::YouGov>::new();

    storage
        .get_data(scope, || async {
            let pdfs = download::download_yougov_data(scope).await?;

            extract::extract_yougov_data(&pdfs)
        })
        .await
}

#[async_trait::async_trait]
impl super::super::Source for super::YouGov {
    const NAME: &'static str = "YouGov";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_yougov(scope).await
    }
}
