use rand;
use tokio::task;

use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use super::db_error::DbError;

pub async fn hash(password: String) -> Result<String, DbError> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| DbError::Password("Failed to hash password".into()))?
            .to_string())
    })
    .await?
}

pub fn verify(password: String, hash: &str) -> Result<bool, DbError> {
    let hash =
        PasswordHash::new(hash).map_err(|_| DbError::Password("Password hash invalid".into()))?;

    let res = Argon2::default().verify_password(password.as_bytes(), &hash);

    match res {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(_) => Err(DbError::Password("Failed to verify password".into())),
    }
}
