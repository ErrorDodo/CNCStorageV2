use crate::models::users::UserLoginDTO;
use crate::routes::account::create_user::create_user;
use crate::routes::account::login_user::login_user;
use crate::{db::DbPool, models::users::UserDTO};
use actix_web::{web, Responder, Scope};
use log::info;

pub fn account_scope() -> Scope {
    web::scope("/account")
        .route("/signup", web::post().to(handle_sign_up))
        .route("/login", web::post().to(handle_login_user))
}

async fn handle_sign_up(payload: web::Json<UserDTO>, pool: web::Data<DbPool>) -> impl Responder {
    info!("Handling add user action");
    create_user(payload, pool).await
}

async fn handle_login_user(
    payload: web::Json<UserLoginDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    info!("Handling login user action");
    login_user(payload, pool).await
}
