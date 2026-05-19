use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn by_id(self, response_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::ResponseId { response_id })
    }
}
