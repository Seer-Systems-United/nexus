use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, source::DatabaseSource},
    schema,
};

pub fn search_sources_by_name(
    source_name: &str,
) -> Result<Vec<DatabaseSource>, diesel::result::Error> {
    debug!(source_name = %source_name, "searching sources by name");

    let mut conn = get_connection();

    match schema::sources::table
        .filter(schema::sources::name.eq(source_name))
        .select(DatabaseSource::as_select())
        .load::<DatabaseSource>(&mut conn)
    {
        Ok(sources) => {
            debug!(count = sources.len(), source_name = %source_name, "found sources by name");
            Ok(sources)
        }
        Err(error) => {
            error!(%error, source_name = %source_name, "error searching sources by name");
            Err(error)
        }
    }
}
