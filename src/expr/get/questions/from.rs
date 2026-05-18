use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn from(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::QuestionFrom { date: date.into() })
    }
}
