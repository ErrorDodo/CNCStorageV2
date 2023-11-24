use crate::db::{establish_connection, DbPool};
use crate::routes::invites::general::handle_invite_actions;
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
    info!("Starting CNC API");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: DbPool = match establish_connection(&database_url) {
        Ok(pool) => pool,
        Err(err) => {
            panic!("Failed to establish database connection: {}", err);
        }
    };
    info!("Database connection established");

    info!("Starting server at http://127.0.0.1:8080");
    get_routes();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/users/{action}", web::route().to(handle_user_actions))
            .route("/invites/{action}", web::route().to(handle_invite_actions))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// TODO: Find a better way to do this
fn get_routes() {
    let mut routes: Vec<String> = Vec::new();

    routes.push("/users/{action}".to_string());
    routes.push("/invites/{action}".to_string());

    for route in routes {
        info!("Registered route: {}", route);
    }
}
