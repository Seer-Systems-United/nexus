pub mod endpoints;
mod query;

pub use endpoints::{
    get_generic_ballot, get_headline_topics, get_important_problem, get_presidential_approval,
    get_right_direction, get_topic, list_topics,
};
pub use query::{HeadlineQuery, TopicQuery, parse_topic_scope};

use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi(
    paths(
        endpoints::headline::list_topics,
        endpoints::dynamic::get_topic,
        endpoints::headline::get_headline_topics,
        endpoints::stable::core::get_presidential_approval,
        endpoints::stable::core::get_right_direction,
        endpoints::stable::issues::get_generic_ballot,
        endpoints::stable::issues::get_important_problem
    ),
    components(schemas(
        crate::topics::types::TopicSummary,
        crate::topics::types::TopicCollection,
        crate::topics::types::TopicObservation,
        crate::topics::types::TopicSource,
        crate::topics::types::TopicStatus,
        crate::topics::types::Compatibility,
        crate::topics::types::DemographicGroup,
        crate::topics::types::DemographicValue,
        crate::topics::types::DemographicResult,
        crate::topics::types::AnswerResult,
        crate::topics::types::PooledAnswerResult,
        crate::topics::types::PooledDemographicResult,
        crate::topics::types::HeadlineTopicSummary,
        crate::sources::Scope,
        crate::api::error::ApiErrorBody,
    )),
    tags((name = "Topics", description = "Canonical polling question topics"))
)]
struct TopicsDoc;

pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(TopicsDoc::openapi())
        .routes(routes!(endpoints::headline::list_topics))
        .routes(routes!(endpoints::headline::get_headline_topics))
        .routes(routes!(endpoints::stable::core::get_presidential_approval))
        .routes(routes!(endpoints::stable::core::get_right_direction))
        .routes(routes!(endpoints::stable::issues::get_generic_ballot))
        .routes(routes!(endpoints::stable::issues::get_important_problem))
        .routes(routes!(endpoints::dynamic::get_topic))
}
