use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn from_poll_id(self, poll_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::QuestionPollId { poll_id })
    }
}
