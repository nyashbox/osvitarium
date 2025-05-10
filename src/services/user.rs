use crate::{app::status::AppStatus as Status, services::utils};

use jsonwebtoken::{EncodingKey, Header, encode};

use crate::models::user::User;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use log::error;

/// Represents JWT token claims
#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    /// Auxiliary Identifier
    pub aux_sub: i32,
    /// User Identifier (ID)
    pub sub: i32,
    /// Issued at
    pub iat: u64,
    /// Expires at
    pub exp: u64,
    /// User role
    pub role: String,
}

#[mockall::automock]
pub trait UserService {
    /// Authenticate user
    ///
    /// # Arguments
    ///
    /// * 'password' - plain-text password
    ///
    /// # Returns
    ///
    /// On success: authenticated user's model
    /// On failure: database error/empty option
    fn authenticate(&self, password: &str) -> Result<(), Status>;

    /// Convert user into JWT
    ///
    /// # Arguments
    ///
    /// * 'secret' - JWT secret
    /// * 'ttl' - Session TTL
    ///
    /// # Returns
    ///
    /// On success: encoded JWT token
    /// On failure: application error
    fn into_jwt(&self, secret: &str, ttl: u64) -> Result<String, Status>;
}

impl UserService for User {
    fn authenticate(&self, password: &str) -> Result<(), Status> {
        let user_model = match self {
            User::Student(model, _) => model,
            User::Teacher(model, _) => model,
            User::Principal(model, _) => model,
        };

        if utils::verify_password(password, &user_model.password)? {
            Ok(())
        } else {
            Err(Status::Unauthenticated(None))
        }
    }

    fn into_jwt(&self, secret: &str, ttl: u64) -> Result<String, Status> {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                error!("Error while getting system time: {e}");

                Status::Internal(None)
            })?
            .as_secs();

        let claims = match self {
            User::Student(user, student) => JWTClaims {
                exp: iat + ttl,
                iat,
                aux_sub: student.student_id,
                sub: user.user_id,
                role: "student".into(),
            },
            User::Teacher(user, teacher) => JWTClaims {
                exp: iat + ttl,
                iat,
                aux_sub: teacher.teacher_id,
                sub: user.user_id,
                role: "teacher".into(),
            },
            User::Principal(user, principal) => JWTClaims {
                exp: iat + ttl,
                iat,
                aux_sub: principal.principal_id,
                sub: user.user_id,
                role: "principal".into(),
            },
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| {
            error!("Error while encoding JWT token: {e}");

            Status::Internal(None)
        })?)
    }
}

#[cfg(test)]
mod tests {}
