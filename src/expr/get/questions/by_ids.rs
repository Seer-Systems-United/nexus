use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn by_ids(self, question_ids: impl IntoIterator<Item = uuid::Uuid>) -> Self {
        self.push_filter(Filter::QuestionIds {
            question_ids: question_ids.into_iter().collect(),
        })
    }
}
