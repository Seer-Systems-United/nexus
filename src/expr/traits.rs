use diesel::PgConnection;

use crate::expr::{
    ExpressionError,
    ops::{Filter, Operation, Table},
};

pub trait OperationTrait {
    const OP: Operation;
}

pub trait TableTrait {
    const TABLE: Table;
}

pub enum FilterApplication<Query> {
    Applied(Query),
    Skipped(Query),
    Empty,
}

impl<Query> FilterApplication<Query> {
    pub fn or_else(
        self,
        apply: impl FnOnce(Query) -> Result<Self, ExpressionError>,
    ) -> Result<Self, ExpressionError> {
        match self {
            Self::Skipped(query) => apply(query),
            result => Ok(result),
        }
    }
}

pub trait FilterTrait<Query> {
    fn apply_filter(
        query: Query,
        filter: &Filter,
        conn: &mut PgConnection,
    ) -> Result<FilterApplication<Query>, ExpressionError>;
}
