pub mod dynamic;
pub mod headline;
pub mod stable;

pub use dynamic::get_topic;
pub use headline::{get_headline_topics, list_topics};
pub use stable::{
    get_generic_ballot, get_important_problem, get_presidential_approval, get_right_direction,
};

use crate::api::error::ApiError;
use crate::api::topics::TopicQuery;
use crate::topics::types::TopicCollection;
use axum::Json;

pub(super) async fn topic_collection(
    topic_id: &str,
    query: &TopicQuery,
) -> Result<Json<TopicCollection>, ApiError> {
    let scope = crate::api::topics::parse_topic_scope(query)?;

    crate::topics::service::get_topic(scope, topic_id)
        .await
        .map(Json)
        .map_err(topic_error)
}

pub(super) fn topic_error(error: Box<dyn std::error::Error + Send + Sync>) -> ApiError {
    if let Some(io_error) = error.downcast_ref::<std::io::Error>()
        && io_error.kind() == std::io::ErrorKind::NotFound
    {
        return ApiError::not_found(io_error.to_string());
    }

    tracing::error!(error = %error, "failed to load topic data");
    ApiError::service_unavailable("topic data unavailable")
}
