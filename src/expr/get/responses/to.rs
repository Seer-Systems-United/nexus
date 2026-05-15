use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn to(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::ResponseTo { date: date.into() })
    }
}
