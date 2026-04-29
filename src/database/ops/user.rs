use crate::database::DbConnection;
use crate::database::schema::user::users::dsl::{
    account_number as user_account_number, email as user_email, users,
};
use crate::database::schema::user::{NewUser, User};
use aws_lc_rs::rand::{SecureRandom, SystemRandom};
use diesel::prelude::*;

const ACCOUNT_NUMBER_LEN: usize = 16;

#[derive(Debug)]
pub enum AccountNumberError {
    Crypto,
}

pub fn create_oidc_user(conn: &mut DbConnection, name: &str, email: &str) -> QueryResult<User> {
    let new_user = NewUser {
        id: uuid::Uuid::new_v4(),
        name: name.to_owned(),
        email: Some(email.to_owned()),
        account_number: None,
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn create_account_number_user(
    conn: &mut DbConnection,
    name: &str,
    account_number: &str,
) -> QueryResult<User> {
    let new_user = NewUser {
        id: uuid::Uuid::new_v4(),
        name: name.to_owned(),
        email: None,
        account_number: Some(account_number.to_owned()),
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

pub fn get_user_by_account_number(
    conn: &mut DbConnection,
    account_number: &str,
) -> QueryResult<User> {
    users
        .filter(user_account_number.eq(account_number))
        .first(conn)
}

pub fn get_or_create_user_by_email(
    conn: &mut DbConnection,
    name: &str,
    email: &str,
) -> QueryResult<User> {
    match get_user_by_email(conn, email) {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::NotFound) => create_oidc_user(conn, name, email),
        Err(error) => Err(error),
    }
}

pub fn generate_account_number() -> Result<String, AccountNumberError> {
    let mut random_bytes = [0u8; ACCOUNT_NUMBER_LEN];
    SystemRandom::new()
        .fill(&mut random_bytes)
        .map_err(|_| AccountNumberError::Crypto)?;

    let mut account_number = String::with_capacity(ACCOUNT_NUMBER_LEN);
    account_number.push(char::from(b'1' + random_bytes[0] % 9));

    for byte in random_bytes.iter().skip(1) {
        account_number.push(char::from(b'0' + byte % 10));
    }

    Ok(account_number)
}

pub fn normalize_account_number(account_number: &str) -> String {
    account_number
        .chars()
        .filter(|character| character.is_ascii_digit())
        .collect()
}

pub fn is_valid_account_number(account_number: &str) -> bool {
    account_number.len() == ACCOUNT_NUMBER_LEN
        && account_number
            .chars()
            .all(|character| character.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::{generate_account_number, is_valid_account_number, normalize_account_number};

    #[test]
    fn account_numbers_are_numeric_identifiers() {
        let account_number = generate_account_number().unwrap();

        assert_eq!(account_number.len(), 16);
        assert!(
            account_number
                .chars()
                .all(|character| character.is_ascii_digit())
        );
        assert_ne!(account_number.as_bytes()[0], b'0');
        assert!(is_valid_account_number(&account_number));
    }

    #[test]
    fn normalizes_grouped_account_numbers() {
        assert_eq!(
            normalize_account_number("1234 5678-9012 3456"),
            "1234567890123456"
        );
    }
}
