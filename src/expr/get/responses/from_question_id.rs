use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Responses},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_question_id(self, question_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::ResponseQuestionId { question_id })
    }
}
