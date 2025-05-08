use log::error;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::app::status::AppStatus as Status;

/// Hash password
///
/// # Arguments
///
/// * 'password' - plain-text password
///
/// # Returns
///
/// Password hash
pub fn hash_password(password: &str) -> Result<String, Status> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Failed to create password hash: {e}");

            Status::Internal(None)
        })?;

    Ok(password_hash.to_string())
}

/// Verify password against password hash
///
/// # Arguments
///
/// * 'password' - plain-text password
/// * 'hash' - password hash
///
/// # Return
///
/// `true` if password matches received hash, 'false' if not
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Status> {
    let hash = PasswordHash::new(hash).map_err(|e| {
        error!("Error while parsing password hash: {e}");

        Status::Internal(None)
    })?;
    let argon2 = Argon2::default();

    Ok(argon2.verify_password(password.as_bytes(), &hash).is_ok())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::success("password", true)]
    fn hash_password_test(#[case] input: &str, #[case] expected: bool) {
        let hash = hash_password(input).unwrap();

        assert_eq!(verify_password(input, &hash).unwrap(), expected);
    }

    #[rstest]
    #[case::success("password", true)]
    #[case::mismatch("mismatch", false)]
    fn verify_password_test(#[case] input: &str, #[case] expected: bool) {
        let password_hash =
            "$argon2id$v=19$m=16,t=2,p=1$cGFzc3dvcmQ$8vDS3rsezOjrur01dF12EA".to_string();

        assert_eq!(verify_password(&input, &password_hash).unwrap(), expected)
    }
}
