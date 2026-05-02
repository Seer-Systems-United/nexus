//! # Password hashing and verification
//!
//! Implements PBKDF2-SHA256 password hashing using aws-lc-rs
//! with random salt generation and constant-time verification.

use super::PasswordError;
use super::format::{encode_password_hash, parse_password_hash, password_iterations};
use aws_lc_rs::pbkdf2;
use aws_lc_rs::rand::{SecureRandom, SystemRandom};

/// Salt length in bytes for password hashing.
const PASSWORD_SALT_LEN: usize = 16;

/// Derived key length in bytes for password hashing.
const PASSWORD_HASH_LEN: usize = 32;

/// Get the hashed representation of a password (public wrapper).
///
/// # Parameters
/// - `password`: Plaintext password to hash.
///
/// # Returns
/// - `Ok(String)`: The encoded password hash string.
///
/// # Errors
/// - `PasswordError::Crypto`: Salt generation or hashing failed.
pub fn get_password_hash(password: &str) -> Result<String, PasswordError> {
    hash_password(password)
}

/// Hash a password with PBKDF2-SHA256 and random salt.
///
/// # Parameters
/// - `password`: Plaintext password to hash.
///
/// # Returns
/// - `Ok(String)`: Encoded hash in format `algorithm$iterations$salt$derived`.
///
/// # Errors
/// - `PasswordError::Crypto`: Salt generation or hashing failed.
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

/// Verify a password against a stored hash using constant-time comparison.
///
/// # Parameters
/// - `password`: Plaintext password to verify.
/// - `stored_hash`: The stored hash string to compare against.
///
/// # Returns
/// - `Ok(true)`: Password matches.
/// - `Ok(false)`: Password does not match.
///
/// # Errors
/// - `PasswordError::InvalidFormat`: Stored hash has invalid format.
/// - `PasswordError::InvalidBase64`: Base64 decoding failed.
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
