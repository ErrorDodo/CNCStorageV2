use std::env;

use crate::{
    db::DbPool,
    models::{jwtmodel::Claims, users::UserDTO},
    utils::generate_auth_token::validate_jwt_token,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::info;

pub fn invite_scope() -> Scope {
    web::scope("/invites").route("/create", web::post().to(handle_create_invite))
}

async fn handle_create_invite(
    auth: BearerAuth,
    payload: web::Json<UserDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    if let Err(response) = validate_jwt_token(auth).await {
        return response;
    }

    HttpResponse::Ok().body(format!("Hello, {}!", payload.username))
}
