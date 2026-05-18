use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use tracing::{debug, instrument};

use crate::{
    database::{common::filter::invalid_filter, person::DatabasePerson},
    expr::{
        ExpressionError,
        ops::{Filter, NameField, Table},
    },
};

use super::{rows::SqlitePerson, schema};

#[instrument(level = "debug", skip(conn))]
pub(super) fn get_people(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabasePerson>, ExpressionError> {
    debug!(?filters, "Starting get_people with filters");
    let mut query = schema::people::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::Name {
                field: NameField::FirstName,
                value,
            } => {
                debug!(field = "FirstName", value, "Applying filter");
                query = query.filter(schema::people::given_name.eq(value))
            }
            Filter::Name {
                field: NameField::Surname,
                value,
            } => {
                debug!(field = "Surname", value, "Applying filter");
                query = query.filter(schema::people::surname.eq(value))
            }
            filter => {
                debug!(?filter, "Invalid filter encountered");
                return invalid_filter(Table::People, filter);
            }
        }
    }

    let people: Result<Vec<DatabasePerson>, ExpressionError> = query
        .load::<SqlitePerson>(conn)?
        .into_iter()
        .map(DatabasePerson::try_from)
        .collect();

    debug!(
        count = people.as_ref().map_or(0, |v| v.len()),
        "Loaded people from database"
    );
    people
}
