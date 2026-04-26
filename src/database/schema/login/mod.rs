use diesel::prelude::*;

use crate::database::schema::user::users;

diesel::table! {
    login(id) {
        id -> Uuid,
        user_id -> Uuid,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(login -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(login, users);

#[derive(Queryable, Selectable)]
#[diesel(table_name = login)]
pub struct Login {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = login)]
pub struct NewLogin {
    pub(crate) id: uuid::Uuid,
    pub(crate) user_id: uuid::Uuid,
    pub(crate) password_hash: String,
    pub(crate) created_at: chrono::NaiveDateTime,
}
