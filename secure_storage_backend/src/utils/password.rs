use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2, PasswordHash, PasswordVerifier};
use rand::rngs::OsRng;

use crate::error::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 8;

pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err("Password is too long".to_string());
    }
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err("Password is too short".to_string());
    }
    Ok(())
}

pub fn hash_password(password: impl Into<String>) -> Result<String, ErrorMessage> {
    let password = password.into();
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ErrorMessage::EmptyPassword);
    }
    let salt = SaltString::generate(OsRng);
    let hash_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ErrorMessage::HashingError)?
        .to_string();
    Ok(hash_password)
}

pub fn compare_password(password: &str, hash: &str) -> Result<bool, ErrorMessage> {
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ErrorMessage::EmptyPassword);
    }

    let parsed_hash = PasswordHash::new(hash).map_err(|_| ErrorMessage::InvalidHashFormat)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}