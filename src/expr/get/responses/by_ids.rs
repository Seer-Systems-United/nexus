use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn by_ids(self, response_ids: impl IntoIterator<Item = uuid::Uuid>) -> Self {
        self.push_filter(Filter::ResponseIds {
            response_ids: response_ids.into_iter().collect(),
        })
    }
}
