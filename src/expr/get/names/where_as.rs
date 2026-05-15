use crate::database::person::DatabasePerson;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, NameField, People},
};

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn where_as(self, field: NameField, value: impl Into<String>) -> Self {
        self.push_filter(Filter::Name {
            field,
            value: value.into(),
        })
    }
}
