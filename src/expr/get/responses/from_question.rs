use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_question(self, question: impl Into<String>) -> Self {
        self.push_filter(Filter::ResponseQuestion {
            question: question.into(),
        })
    }
}
