use std::num::NonZeroU32;

use aws_lc_rs::pbkdf2;
use aws_lc_rs::rand::{SecureRandom, SystemRandom};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use diesel::prelude::*;

use crate::database::DbConnection;
use crate::database::schema::login::login::dsl::{
    created_at as login_created_at, login, user_id as login_user_id,
};
use crate::database::schema::login::{Login, NewLogin};

// Using PBKDF2 to comply with FIPS 180-4: https://csrc.nist.gov/publications/detail/fips/180/4/final
const PASSWORD_HASH_ALGORITHM: &str = "pbkdf2-sha256";
// Recommendation by OWASP is 600,000 iterations: https://www.owasp.org/index.php/Password_Storage_Cheat_Sheet#Recommended_Iterations
const PASSWORD_HASH_ITERATIONS: u32 = 600_000;
const PASSWORD_SALT_LEN: usize = 16;
const PASSWORD_HASH_LEN: usize = 32;

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

pub fn get_password_hash(password: &str) -> Result<String, PasswordError> {
    hash_password(password)
}

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let mut salt = [0u8; PASSWORD_SALT_LEN];
    SystemRandom::new()
        .fill(&mut salt)
        .map_err(|_| PasswordError::Crypto)?;

    let mut derived = [0u8; PASSWORD_HASH_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        password_iterations(),
        &salt,
        password.as_bytes(),
        &mut derived,
    );

    Ok(format!(
        "{}${}${}${}",
        PASSWORD_HASH_ALGORITHM,
        PASSWORD_HASH_ITERATIONS,
        BASE64.encode(salt),
        BASE64.encode(derived)
    ))
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, PasswordError> {
    let parsed = parse_password_hash(stored_hash)?;

    Ok(pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA256,
        parsed.iterations,
        &parsed.salt,
        password.as_bytes(),
        &parsed.derived,
    )
    .is_ok())
}

struct ParsedPasswordHash {
    iterations: NonZeroU32,
    salt: Vec<u8>,
    derived: Vec<u8>,
}

fn parse_password_hash(stored_hash: &str) -> Result<ParsedPasswordHash, PasswordError> {
    let mut parts = stored_hash.split('$');
    let algorithm = parts.next().ok_or(PasswordError::InvalidFormat)?;
    let iterations = parts.next().ok_or(PasswordError::InvalidFormat)?;
    let salt = parts.next().ok_or(PasswordError::InvalidFormat)?;
    let derived = parts.next().ok_or(PasswordError::InvalidFormat)?;

    if parts.next().is_some() || algorithm != PASSWORD_HASH_ALGORITHM {
        return Err(PasswordError::InvalidFormat);
    }

    let iterations = iterations
        .parse::<u32>()
        .ok()
        .and_then(NonZeroU32::new)
        .ok_or(PasswordError::InvalidIterations)?;

    Ok(ParsedPasswordHash {
        iterations,
        salt: BASE64.decode(salt).map_err(PasswordError::InvalidBase64)?,
        derived: BASE64
            .decode(derived)
            .map_err(PasswordError::InvalidBase64)?,
    })
}

fn password_iterations() -> NonZeroU32 {
    NonZeroU32::new(PASSWORD_HASH_ITERATIONS).expect("password iterations must be non-zero")
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn hashes_and_verifies_passwords() {
        let stored_hash = hash_password("correct horse battery staple").unwrap();

        assert!(verify_password("correct horse battery staple", &stored_hash).unwrap());
        assert!(!verify_password("wrong password", &stored_hash).unwrap());
    }
}
