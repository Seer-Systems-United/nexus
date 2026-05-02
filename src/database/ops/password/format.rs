use super::PasswordError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::num::NonZeroU32;

pub(super) const PASSWORD_HASH_ALGORITHM: &str = "pbkdf2-sha256";
pub(super) const PASSWORD_HASH_ITERATIONS: u32 = 600_000;

pub(super) struct ParsedPasswordHash {
    pub(super) iterations: NonZeroU32,
    pub(super) salt: Vec<u8>,
    pub(super) derived: Vec<u8>,
}

pub(super) fn password_iterations() -> NonZeroU32 {
    NonZeroU32::new(PASSWORD_HASH_ITERATIONS).expect("password iterations must be non-zero")
}

pub(super) fn encode_password_hash(salt: &[u8], derived: &[u8]) -> String {
    format!(
        "{}${}${}${}",
        PASSWORD_HASH_ALGORITHM,
        PASSWORD_HASH_ITERATIONS,
        BASE64.encode(salt),
        BASE64.encode(derived)
    )
}

pub(super) fn parse_password_hash(stored_hash: &str) -> Result<ParsedPasswordHash, PasswordError> {
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
