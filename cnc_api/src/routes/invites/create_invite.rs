use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use log::{error, info};
use uuid::Uuid;

use crate::{
    db::DbPool,
    models::{invites::NewInvite, jwtmodel::Claims},
    schema::{invites, users},
};

pub async fn build_invite(
    payload: Claims,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user_id: Uuid = users::table
        .filter(users::columns::username.eq(&payload.username))
        .select(users::columns::user_id)
        .first(&mut *conn)
        .map_err(|e| {
            error!("Failed to find user: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to find user")
        })?;

    let invite_code = Uuid::new_v4().to_string();

    let new_invite = NewInvite {
        generated_by_user_id: user_id,
        has_been_used: false,
        date_used: None,
        used_by_user_id: None,
        invite_code: &invite_code,
    };

    match diesel::insert_into(invites::table)
        .values(&new_invite)
        .execute(&mut *conn)
    {
        Ok(_) => {
            info!("Invite created successfully for user: {}", payload.username);
            Ok(HttpResponse::Ok().json(invite_code))
        }
        Err(e) => {
            error!("Error creating invite: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
