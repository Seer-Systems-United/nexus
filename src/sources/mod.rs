pub mod data;
pub mod date;
pub mod emerson;
pub mod gallup;
pub mod ipsos;
pub mod persistance;
pub mod registry;
pub mod scope;
pub mod yougov;

use std::error::Error;

pub use data::*;
pub use registry::SourceId;
pub use scope::Scope;

#[async_trait::async_trait]
pub trait Source {
    const NAME: &'static str;
    const CACHE_VERSION: &'static str = "v1";

    async fn get_data(scope: Scope) -> Result<DataCollection, Box<dyn Error + Send + Sync>>;
}
