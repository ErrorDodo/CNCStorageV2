use actix_web::{web, HttpResponse, Responder, Scope};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::info;

use crate::{
    db::DbPool,
    routes::files::pictures::get_all_pictures,
    utils::{errors::auth::AuthError, generate_auth_token::validate_jwt_token},
};

pub fn files_scope() -> Scope {
    web::scope("/files").route("/images/getall", web::get().to(handle_get_all_pictures))
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
