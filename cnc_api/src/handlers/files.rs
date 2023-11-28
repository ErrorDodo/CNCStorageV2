use actix_web::{web, HttpResponse, Responder, Scope};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::info;
use uuid::Uuid;

use crate::{
    db::DbPool,
    routes::files::pictures::{get_all_pictures, get_pictures_by_user, recent_upload_pictures},
    utils::{errors::auth::AuthError, generate_auth_token::validate_jwt_token},
};

pub fn files_scope() -> Scope {
    web::scope("/files")
        .route("/images/recent", web::get().to(handle_get_recent_uploads))
        .route("/images/getall", web::get().to(handle_get_all_pictures))
        .route("/images/{id}", web::get().to(handle_get_user_pictures))
}

async fn handle_get_all_pictures(
    auth: Option<BearerAuth>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match validate_jwt_token(auth).await {
        Ok(claims) => match get_all_pictures(pool).await {
            Ok(response) => {
                info!("User {} authenticated", claims.username);
                response
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(AuthError::JwtNotFound) => HttpResponse::Unauthorized().body("JWT token not found"),
        Err(AuthError::JwtInvalid) => HttpResponse::Unauthorized().body("Invalid JWT token"),
    }
}

async fn handle_get_user_pictures(
    user_id: web::Path<Uuid>,
    auth: Option<BearerAuth>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match validate_jwt_token(auth).await {
        Ok(claims) => {
            info!("User {} authenticated", claims.username);
            match get_pictures_by_user(pool, user_id).await {
                Ok(response) => response,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(AuthError::JwtNotFound) => HttpResponse::Unauthorized().body("JWT token not found"),
        Err(AuthError::JwtInvalid) => HttpResponse::Unauthorized().body("Invalid JWT token"),
    }
}

async fn handle_get_recent_uploads(
    auth: Option<BearerAuth>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match validate_jwt_token(auth).await {
        Ok(claims) => {
            info!("User {} authenticated", claims.username);
            match recent_upload_pictures(pool).await {
                Ok(response) => response,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(AuthError::JwtNotFound) => HttpResponse::Unauthorized().body("JWT token not found"),
        Err(AuthError::JwtInvalid) => HttpResponse::Unauthorized().body("Invalid JWT token"),
    }
}
