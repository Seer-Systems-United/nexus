//! # Database operations tests
//!
//! Tests for database password hashing, verification, and user operations.

use nexus::database::ops::password::{hash_password, verify_password};
use nexus::database::ops::user::{
    generate_account_number, is_valid_account_number, normalize_account_number,
};

#[test]
fn hashes_and_verifies_passwords() {
    let stored_hash = hash_password("correct horse battery staple").unwrap();

    assert!(verify_password("correct horse battery staple", &stored_hash).unwrap());
    assert!(!verify_password("wrong password", &stored_hash).unwrap());
}

#[test]
fn account_numbers_are_numeric_identifiers() {
    let account_number = generate_account_number().unwrap();

    assert_eq!(account_number.len(), 16);
    assert!(account_number.chars().all(|ch| ch.is_ascii_digit()));
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
