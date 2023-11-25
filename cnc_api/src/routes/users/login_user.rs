use crate::models::users::User;
use crate::schema::users::dsl::*;
use crate::utils::generate_auth_token::generate_jwt;
use crate::{db::DbPool, models::users::UserLoginDTO};
use actix_web::{web, HttpResponse, Result};
use bcrypt::verify;
use diesel::prelude::*;
use log::info;

pub async fn login_user(
    login_dto: web::Json<UserLoginDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match users
        .filter(username.eq(&login_dto.username))
        .first::<User>(&mut *conn)
    {
        Ok(user) => {
            if verify(&login_dto.password, &user.password_hash).unwrap_or(false) {
                let token = generate_jwt(user.id);

                Ok(HttpResponse::Ok().json(token))
            } else {
                info!("Login failed: Invalid username or password");
                Ok(HttpResponse::Unauthorized().body("Invalid username or password"))
            }
        }
        Err(e) => {
            info!("Login failed: {}", e);
            Ok(HttpResponse::Unauthorized().body("Invalid username or password"))
        }
    }
}
