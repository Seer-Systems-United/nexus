pub mod create;
pub mod get;
pub mod search;
pub mod update;

use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use crate::schema::response_units;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = response_units)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseResponseUnit {
    pub id: uuid::Uuid,
    pub name: String,
}
