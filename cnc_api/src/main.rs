use crate::db::{establish_connection, DbPool};
use crate::routes::users::general::handle_user_actions;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

mod db;
mod models;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: DbPool = establish_connection(&database_url);
    info!("Database connection established");

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/users/{action}", web::route().to(handle_user_actions))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
