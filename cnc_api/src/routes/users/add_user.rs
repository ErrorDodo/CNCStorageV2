use crate::models::users::NewUser;
use crate::schema::users;
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

    let hashed_password =
        hash(user_dto.password.clone(), DEFAULT_COST).expect("Error hashing password");

    let new_user = NewUser {
        username: &user_dto.username,
        password_hash: &hashed_password,
        date_registered: chrono::Utc::now().naive_utc(),
        invited_by_user_id: None,
        is_admin: false,
        is_moderator: false,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut *conn)
    {
        Ok(_) => {
            info!("User created successfully: {}", user_dto.username,);
            Ok(HttpResponse::Ok().json("User created successfully"))
        }
        Err(e) => {
            error!("Error creating user: {}", e);
            Ok(HttpResponse::InternalServerError().body(format!("Error creating user: {}", e)))
        }
    }
}
