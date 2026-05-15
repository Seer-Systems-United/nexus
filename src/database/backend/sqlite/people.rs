use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{
    database::{common::filter::invalid_filter, person::DatabasePerson},
    expr::{
        ExpressionError,
        ops::{Filter, NameField, Table},
    },
};

use super::{rows::SqlitePerson, schema};

pub(super) fn get_people(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabasePerson>, ExpressionError> {
    let mut query = schema::people::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::Name {
                field: NameField::FirstName,
                value,
            } => query = query.filter(schema::people::given_name.eq(value)),
            Filter::Name {
                field: NameField::Surname,
                value,
            } => query = query.filter(schema::people::surname.eq(value)),
            filter => return invalid_filter(Table::People, filter),
        }
    }

    query
        .load::<SqlitePerson>(conn)?
        .into_iter()
        .map(DatabasePerson::try_from)
        .collect()
}
