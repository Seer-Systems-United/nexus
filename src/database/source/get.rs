use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, source::DatabaseSource},
    schema,
};

pub fn get_source_by_id(id: uuid::Uuid) -> Result<DatabaseSource, diesel::result::Error> {
    debug!(source_id = %id, "getting source by id");

    let mut conn = get_connection();

    match schema::sources::table
        .find(id)
        .select(DatabaseSource::as_select())
        .first::<DatabaseSource>(&mut conn)
    {
        Ok(source) => {
            debug!(source_id = %id, "found source");
            Ok(source)
        }
        Err(error) => {
            error!(%error, source_id = %id, "error finding source by id");
            Err(error)
        }
    }
}
