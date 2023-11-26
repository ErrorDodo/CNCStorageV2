use crate::models::jwtmodel::Claims;
use jsonwebtoken::{decode, errors::Error as JwtError, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use super::errors::auth::AuthError;

pub fn generate_jwt(user_id: Uuid) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 60 * 60; // For example, 1 hour expiry

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Failed to generate token")
}

pub fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub fn extract_jwt_from_request(request: &actix_web::HttpRequest) -> Result<String, AuthError> {
    if let Some(auth_header) = request.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Ok(auth_str[7..].to_string());
            }
        }
    }
    Err(AuthError::new("JWT not found in the Authorization header"))
}
