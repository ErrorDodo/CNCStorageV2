use actix_web::{web, HttpResponse};

use crate::{db::DbPool, models::invites::CreateInviteDTO};

pub async fn build_invite(
    payload: web::Json<CreateInviteDTO>,
    _pool: web::Data<DbPool>,
) -> HttpResponse {
    HttpResponse::Ok().body(format!("Creating invite for user: {}", payload.username))
}
