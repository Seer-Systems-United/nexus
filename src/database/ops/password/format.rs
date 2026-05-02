//! # Password hash format utilities
//!
//! Handles encoding and parsing of password hashes in the format:
//! `algorithm$iterations$salt$derived`
//! Currently supports PBKDF2-SHA256 with base64-encoded components.

use super::PasswordError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::num::NonZeroU32;

/// Password hash algorithm identifier.
pub(super) const PASSWORD_HASH_ALGORITHM: &str = "pbkdf2-sha256";

/// Default number of PBKDF2 iterations.
pub(super) const PASSWORD_HASH_ITERATIONS: u32 = 600_000;

/// Parsed components of a stored password hash.
///
/// # Fields
/// - `iterations`: Number of PBKDF2 iterations.
/// - `salt`: The salt bytes used for hashing.
/// - `derived`: The derived key bytes.
pub(super) struct ParsedPasswordHash {
    pub(super) iterations: NonZeroU32,
    pub(super) salt: Vec<u8>,
    pub(super) derived: Vec<u8>,
}

/// Get the configured password iteration count as a non-zero value.
pub(super) fn password_iterations() -> NonZeroU32 {
    NonZeroU32::new(PASSWORD_HASH_ITERATIONS).expect("password iterations must be non-zero")
}

/// Encode salt and derived key into the standard hash format string.
///
/// # Parameters
/// - `salt`: The salt bytes.
/// - `derived`: The derived key bytes.
///
/// # Returns
/// - Format: `pbkdf2-sha256$600000$<base64_salt>$<base64_derived>`
pub(super) fn encode_password_hash(salt: &[u8], derived: &[u8]) -> String {
    format!(
        "{}${}${}${}",
        PASSWORD_HASH_ALGORITHM,
        PASSWORD_HASH_ITERATIONS,
        BASE64.encode(salt),
        BASE64.encode(derived)
    )
}

/// Parse a stored password hash string into its components.
///
/// # Parameters
/// - `stored_hash`: The hash string to parse.
///
/// # Returns
/// - `Ok(ParsedPasswordHash)`: The parsed hash components.
///
/// # Errors
/// - `PasswordError::InvalidFormat`: Hash format is invalid.
/// - `PasswordError::InvalidIterations`: Iteration count is invalid.
/// - `PasswordError::InvalidBase64`: Base64 decoding failed.
pub(super) fn parse_password_hash(stored_hash: &str) -> Result<ParsedPasswordHash, PasswordError> {
    let mut parts = stored_hash.split('$');
    let algorithm = parts.next().ok_or(PasswordError::InvalidFormat)?;
    let iterations = parts.next().ok_or(PasswordError::InvalidFormat)?;
    let salt = parts.next().ok_or(PasswordError::InvalidFormat)?;
    let derived = parts.next().ok_or(PasswordError::InvalidFormat)?;

    // Ensure there are no extra parts and algorithm matches
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
