use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn from_source_id(self, source_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::QuestionSourceId { source_id })
    }
}
