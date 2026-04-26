use crate::database::DbConnection;
use crate::database::schema::user::users::dsl::{email as user_email, users};
use crate::database::schema::user::{NewUser, User};
use diesel::prelude::*;

pub fn create_user(conn: &mut DbConnection, name: &str, email: &str) -> QueryResult<User> {
    let new_user = NewUser {
        id: uuid::Uuid::new_v4(),
        name: name.to_owned(),
        email: email.to_owned(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user_by_id(conn: &mut DbConnection, id: &uuid::Uuid) -> QueryResult<User> {
    users.find(*id).first(conn)
}

pub fn get_user_by_email(conn: &mut DbConnection, email: &str) -> QueryResult<User> {
    users.filter(user_email.eq(email)).first(conn)
}
