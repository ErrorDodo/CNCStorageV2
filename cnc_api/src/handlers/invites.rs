use crate::{
    db::DbPool,
    routes::invites::create_invite::build_invite,
    utils::{errors::auth::AuthError, generate_auth_token::validate_jwt_token},
};
use actix_web::{web, HttpResponse, Responder, Scope};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub fn invite_scope() -> Scope {
    web::scope("/invites").route("/create", web::post().to(handle_create_invite))
}

async fn handle_create_invite(auth: Option<BearerAuth>, pool: web::Data<DbPool>) -> impl Responder {
    match validate_jwt_token(auth).await {
        Ok(claims) => match build_invite(claims, pool).await {
            Ok(response) => response,
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(AuthError::JwtNotFound) => HttpResponse::Unauthorized().body("JWT token not found"),
        Err(AuthError::JwtInvalid) => HttpResponse::Unauthorized().body("Invalid JWT token"),
    }
}
