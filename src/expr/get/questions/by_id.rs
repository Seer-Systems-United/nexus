use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn by_id(self, question_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::QuestionId { question_id })
    }
}
