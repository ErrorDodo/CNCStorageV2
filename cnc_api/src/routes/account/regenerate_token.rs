use actix_web::{web, HttpResponse};

use crate::{
    models::jwtmodel::{AuthTokens, RefreshTokenRequest},
    utils::generate_auth_token::{generate_jwt, generate_refresh_token, validate_refresh_token},
};

pub async fn refresh_token(
    refresh_token: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let refresh_token = &refresh_token.refresh_token;

    match validate_refresh_token(refresh_token) {
        Ok(claims) => {
            let user_id = claims.sub;
            let username = claims.username;

            let new_access_token = generate_jwt(user_id, username.clone());
            let new_refresh_token = generate_refresh_token(user_id, username);

            let tokens = AuthTokens {
                access_token: new_access_token,
                refresh_token: new_refresh_token,
            };

            Ok(HttpResponse::Ok().json(tokens))
        }
        Err(e) => Ok(HttpResponse::Unauthorized().body("Invalid refresh token")),
    }
}
