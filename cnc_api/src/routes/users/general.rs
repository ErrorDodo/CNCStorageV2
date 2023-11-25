use crate::models::users::UserLoginDTO;
use crate::routes::users::add_user::create_user;
use crate::routes::users::login_user::login_user;
use crate::{db::DbPool, models::users::UserDTO};
use actix_web::{web, Responder};
use log::info;

pub async fn handle_add_user(
    payload: web::Json<UserDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    info!("Handling add user action");
    create_user(payload, pool).await
}

pub async fn handle_login_user(
    payload: web::Json<UserLoginDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    info!("Handling login user action");
    login_user(payload, pool).await
}
