pub mod core;
pub mod issues;

pub use core::{get_presidential_approval, get_right_direction};
pub use issues::{get_generic_ballot, get_important_problem};

use crate::api::error::ApiError;
use crate::api::topics::TopicQuery;
use crate::topics::types::TopicCollection;
use axum::Json;

async fn stable_topic_endpoint(
    topic_id: &'static str,
    query: TopicQuery,
) -> Result<Json<TopicCollection>, ApiError> {
    super::topic_collection(topic_id, &query).await
}
