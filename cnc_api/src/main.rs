use crate::db::{establish_connection, DbPool};
use crate::handlers::invites::invite_scope;
use crate::handlers::users::user_scope;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

mod db;
mod handlers;
mod middleware;
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user_scope())
            .service(invite_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
