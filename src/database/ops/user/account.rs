use aws_lc_rs::rand::{SecureRandom, SystemRandom};

const ACCOUNT_NUMBER_LEN: usize = 16;

#[derive(Debug)]
pub enum AccountNumberError {
    Crypto,
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
