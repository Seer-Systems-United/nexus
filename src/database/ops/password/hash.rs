use super::PasswordError;
use super::format::{encode_password_hash, parse_password_hash, password_iterations};
use aws_lc_rs::pbkdf2;
use aws_lc_rs::rand::{SecureRandom, SystemRandom};

const PASSWORD_SALT_LEN: usize = 16;
const PASSWORD_HASH_LEN: usize = 32;

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

    Ok(encode_password_hash(&salt, &derived))
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
