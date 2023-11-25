use crate::db::{establish_connection, DbPool};
use crate::routes::invites::general::handle_invite_actions;
use crate::routes::users::general::{handle_add_user, handle_login_user};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

mod db;
mod models;
mod routes;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: DbPool =
        establish_connection(&database_url).expect("Failed to establish database connection");
    info!("Database connection established");

    info!("Starting server at http://127.0.0.1:8080");
    info!("Registered route: /users/add");
    info!("Registered route: /users/login");
    info!("Registered route: /invites");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/users/add", web::post().to(handle_add_user))
            .route("/users/login", web::post().to(handle_login_user))
            .route("/invites", web::route().to(handle_invite_actions))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
