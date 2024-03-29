use crate::models::jwtmodel::Claims;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use super::errors::auth::AuthError;

pub fn generate_jwt(user_id: Uuid, username: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 60 * 60; // For example, 1 hour expiry

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
        username: username,
    };

    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Failed to generate token")
}

pub fn generate_refresh_token(user_id: Uuid, username: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 60 * 60 * 24 * 30; // For example, 30 days expiry

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
        username: username,
    };

    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Failed to generate token")
}

pub async fn validate_jwt_token(auth: Option<BearerAuth>) -> Result<Claims, AuthError> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let validation = Validation::default();

    let token = match auth {
        Some(t) => t.token().to_owned(),
        None => return Err(AuthError::JwtNotFound),
    };

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(error) => match *error.kind() {
            ErrorKind::InvalidToken => Err(AuthError::JwtInvalid), // Token is invalid
            ErrorKind::ExpiredSignature => Err(AuthError::JwtInvalid), // Expired Token
            _ => Err(AuthError::JwtInvalid),                       // Some other error
        },
    }
}

pub fn validate_refresh_token(token: &str) -> Result<Claims, AuthError> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let validation = Validation::default();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(error) => match *error.kind() {
            ErrorKind::InvalidToken => Err(AuthError::JwtInvalid), // Token is invalid
            ErrorKind::ExpiredSignature => Err(AuthError::JwtInvalid), // Expired Token
            _ => Err(AuthError::JwtInvalid),                       // Some other error
        },
    }
}

// Might be needed in the future

// pub fn extract_jwt_from_request(request: &actix_web::HttpRequest) -> Result<String, AuthError> {
//     if let Some(auth_header) = request.headers().get("Authorization") {
//         if let Ok(auth_str) = auth_header.to_str() {
//             if auth_str.starts_with("Bearer ") {
//                 return Ok(auth_str[7..].to_string());
//             }
//         }
//     }
//     Err(AuthError::new("JWT not found in the Authorization header"))
// }
