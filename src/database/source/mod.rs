pub mod create;
pub mod get;
pub mod search;
pub mod update;

use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use crate::schema::sources;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseSource {
    pub id: uuid::Uuid,
    pub name: String,
}
