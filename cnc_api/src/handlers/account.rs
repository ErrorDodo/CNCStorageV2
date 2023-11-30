use crate::models::jwtmodel::RefreshTokenRequest;
use crate::models::users::UserLoginDTO;
use crate::routes::account::create_user::create_user;
use crate::routes::account::login_user::login_user;
use crate::routes::account::regenerate_token::refresh_token;
use crate::utils::errors::auth::AuthError;
use crate::utils::generate_auth_token::{validate_jwt_token, validate_refresh_token};
use crate::utils::log_db::log_event;
use crate::{db::DbPool, models::users::UserDTO};
use actix_web::{web, HttpResponse, Responder, Scope};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::error;

pub fn account_scope() -> Scope {
    web::scope("/account")
        .route("/signup", web::post().to(handle_sign_up))
        .route("/login", web::post().to(handle_login_user))
        .route("/refresh_token", web::get().to(handle_regenerate_token))
        .route("/logout", web::get().to(handle_logout_user))
}

async fn handle_sign_up(payload: web::Json<UserDTO>, pool: web::Data<DbPool>) -> impl Responder {
    create_user(payload, pool).await
}

async fn handle_login_user(
    payload: web::Json<UserLoginDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    login_user(payload, pool).await
}

async fn handle_regenerate_token(
    refresh_token_request: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    match refresh_token(refresh_token_request).await {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn handle_logout_user(auth: Option<BearerAuth>, pool: web::Data<DbPool>) -> impl Responder {
    match validate_jwt_token(auth).await {
        Ok(claims) => {
            let result = log_event(
                pool.clone(),
                "Logout",
                Some(claims.sub),
                &format!("User {} logged out", claims.username),
            )
            .await;

            if let Err(e) = result {
                error!("Failed to log logout event: {}", e);
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Ok().body("Please delete your tokens")
        }
        Err(AuthError::JwtNotFound) => HttpResponse::Unauthorized().body("JWT token not found"),
        Err(AuthError::JwtInvalid) => HttpResponse::Unauthorized().body("Invalid JWT token"),
    }
}
