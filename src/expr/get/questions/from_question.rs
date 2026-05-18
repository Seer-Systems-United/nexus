use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn from_question(self, question: impl Into<String>) -> Self {
        self.push_filter(Filter::QuestionQuestion {
            question: question.into(),
        })
    }
}
