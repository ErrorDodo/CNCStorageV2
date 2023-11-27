use crate::models::invites::Invite;
use crate::models::users::NewUser;
use crate::schema::{invites, users};
use crate::{db::DbPool, models::users::UserDTO};
use actix_web::{web, HttpResponse, Result};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use log::{error, info};

pub async fn create_user(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let invite_result = invites::table
        .filter(invites::columns::invite_code.eq(&user_dto.invite_code))
        .first::<Invite>(&mut *conn);

    match invite_result {
        Ok(inv) if !inv.has_been_used => {
            let hashed_password =
                hash(user_dto.password.clone(), DEFAULT_COST).expect("Error hashing password");

            let new_user = NewUser {
                username: &user_dto.username,
                password_hash: &hashed_password,
                date_registered: chrono::Utc::now().naive_utc(),
                invited_by_user_id: Some(inv.generated_by_user_id),
                is_admin: false,
                is_moderator: false,
            };

            match diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut *conn)
            {
                Ok(_) => {
                    info!("User created successfully: {}", user_dto.username);
                    Ok(HttpResponse::Ok().json("User created successfully"))
                }
                Err(e) => {
                    error!("Error creating user: {}", e);
                    Ok(HttpResponse::InternalServerError()
                        .body(format!("Error creating user: {}", e)))
                }
            }
        }
        Ok(_) => Ok(HttpResponse::BadRequest().body("Invite code has already been used")),
        Err(e) => {
            error!("Error finding invite: {}", e);
            Ok(HttpResponse::BadRequest().body("Invalid invite code"))
        }
    }
}
