use crate::{db::DbPool, models::users::UserDTO};
use actix_web::{web, Responder, Scope};
use log::info;

pub fn invite_scope() -> Scope {
    web::scope("/invites").route("/add", web::post().to(handle_create_invite))
}

// Handle create invite, a jwt token is required
async fn handle_create_invite(
    payload: web::Json<UserDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    "Not implemented"
}
