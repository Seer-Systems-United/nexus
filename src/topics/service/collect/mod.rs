use crate::sources::{Scope, SourceId};
use crate::topics::service::{MappedSourceData, map};

pub async fn collect_unenriched_source_data(scope: Scope) -> MappedSourceData {
    let mut observations = Vec::new();
    let mut warnings = Vec::new();

    for source in SourceId::ALL {
        match source.load(scope).await {
            Ok(collection) => {
                observations.extend(map::map_source_collection(source, &collection));
            }
            Err(error) => {
                warnings.push(format!("{} unavailable: {error}", source.name()));
                tracing::warn!(
                    source = source.id(),
                    error = %error,
                    "failed to load topic source data"
                );
            }
        }
    }

    MappedSourceData {
        observations,
        warnings,
    }
}
