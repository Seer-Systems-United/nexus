use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn from_poll_ids(self, poll_ids: impl IntoIterator<Item = uuid::Uuid>) -> Self {
        self.push_filter(Filter::QuestionPollIds {
            poll_ids: poll_ids.into_iter().collect(),
        })
    }
}
