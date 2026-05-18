use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn to(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::QuestionTo { date: date.into() })
    }
}
