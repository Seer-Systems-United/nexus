pub mod create;
pub mod get;
pub mod search;
pub mod update;

use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use crate::schema::people;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = people)]
pub struct Person {
    pub id: uuid::Uuid,
    pub given_name: String,
    pub surname: String,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
}
