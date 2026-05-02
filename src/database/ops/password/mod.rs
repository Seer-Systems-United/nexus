mod format;
mod hash;

pub use hash::{get_password_hash, hash_password, verify_password};

use crate::database::DbConnection;
use crate::database::schema::login::login::dsl::{
    created_at as login_created_at, login, user_id as login_user_id,
};
use crate::database::schema::login::{Login, NewLogin};
use diesel::prelude::*;

#[derive(Debug)]
pub enum PasswordError {
    Crypto,
    InvalidFormat,
    InvalidBase64(base64::DecodeError),
    InvalidIterations,
    Database(diesel::result::Error),
}

impl From<diesel::result::Error> for PasswordError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Database(error)
    }
}

pub fn create_login(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
    password: &str,
) -> Result<Login, PasswordError> {
    let password_hash = hash_password(password)?;
    let new_login = NewLogin {
        id: uuid::Uuid::new_v4(),
        user_id,
        password_hash,
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(login)
        .values(&new_login)
        .get_result(conn)
        .map_err(PasswordError::from)
}

pub fn get_latest_login_for_user(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
) -> QueryResult<Login> {
    login
        .filter(login_user_id.eq(user_id))
        .order(login_created_at.desc())
        .first(conn)
}

pub fn verify_login_password(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
    password: &str,
) -> Result<bool, PasswordError> {
    let login_row = get_latest_login_for_user(conn, user_id)?;

    verify_password(password, &login_row.password_hash)
}
