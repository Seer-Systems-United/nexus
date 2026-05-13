use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error, instrument};

use crate::{
    database::{get_connection, source::DatabaseSource},
    schema,
};

fn create_source(name: &str) -> DatabaseSource {
    DatabaseSource {
        id: uuid::Uuid::new_v4(),
        name: name.to_string(),
    }
}

#[instrument(level = "info", skip_all, fields(source_name = %name))]
pub fn create_source_in_db(name: &str) -> Result<DatabaseSource, diesel::result::Error> {
    debug!("creating source");

    let mut conn = get_connection();

    match schema::sources::table
        .filter(schema::sources::name.eq(name))
        .select(DatabaseSource::as_select())
        .first::<DatabaseSource>(&mut conn)
        .optional()
    {
        Ok(Some(source)) => {
            debug!(source_id = %source.id, "source already exists");
            return Ok(source);
        }
        Ok(None) => {}
        Err(error) => {
            error!(%error, "error checking for existing source");
            return Err(error);
        }
    }

    let new_source = create_source(name);

    match diesel::insert_into(schema::sources::table)
        .values(&new_source)
        .returning(DatabaseSource::as_returning())
        .get_result(&mut conn)
    {
        Ok(source) => {
            debug!(source_id = %source.id, "inserted source");
            Ok(source)
        }
        Err(error) => {
            error!(%error, source_id = %new_source.id, "error inserting source");
            Err(error)
        }
    }
}
