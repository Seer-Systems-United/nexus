use diesel::prelude::*;

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub(crate) id: uuid::Uuid,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) created_at: chrono::NaiveDateTime,
}
