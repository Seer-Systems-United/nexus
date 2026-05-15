use crate::database::person::DatabasePerson;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, NameField, People, Table},
};

impl NexusExpression<GetOp> {
    pub fn names(self) -> NexusExpression<GetOp, People, DatabasePerson> {
        self.select_table(Table::People)
    }
}

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn where_as(self, field: NameField, value: impl Into<String>) -> Self {
        self.push_filter(Filter::Name {
            field,
            value: value.into(),
        })
    }
}
