use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses, SourceFilter},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_source<S: SourceFilter>(self, _source: S) -> Self {
        self.push_filter(Filter::ResponseSource {
            source_name: S::SOURCE_NAME,
        })
    }
}
