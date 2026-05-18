use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_source_id(self, source_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::ResponseSourceId { source_id })
    }
}
