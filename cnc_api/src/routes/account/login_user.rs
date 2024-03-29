use crate::{
    db::DbPool,
    models::{
        jwtmodel::AuthTokens,
        users::{User, UserLoginDTO},
    },
    schema::users::dsl::*,
    utils::{
        generate_auth_token::{generate_jwt, generate_refresh_token},
        log_db::log_event,
    },
};
use actix_web::{web, HttpResponse, Result};
use bcrypt::verify;
use diesel::prelude::*;
use log::{error, info};

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
                let token = generate_jwt(user.id, login_dto.username.clone());
                let refresh_token = generate_refresh_token(user.id, login_dto.username.clone());

                let return_json = AuthTokens {
                    access_token: token,
                    refresh_token: refresh_token,
                };

                // Log successful login
                log_event(
                    pool.clone(),
                    "Login Success",
                    Some(user.id),
                    &format!("User {} logged in successfully", login_dto.username),
                )
                .await?;

                Ok(HttpResponse::Ok().json(return_json))
            } else {
                info!("Login failed: Invalid username or password");

                // Log failed login attempt
                log_event(
                    pool.clone(),
                    "Login Failure",
                    None,
                    &format!("Failed login attempt for username {}", login_dto.username),
                )
                .await?;

                Ok(HttpResponse::Unauthorized().body("Invalid username or password"))
            }
        }
        Err(e) => {
            error!("Login failed: {}", e);
            error!("Login for user {} failed", login_dto.username);

            // Log failed login attempt
            log_event(
                pool.clone(),
                "Login Failure",
                None,
                &format!("Failed login attempt for username {}", login_dto.username),
            )
            .await?;

            Ok(HttpResponse::Unauthorized().body("Invalid username or password"))
        }
    }
}
