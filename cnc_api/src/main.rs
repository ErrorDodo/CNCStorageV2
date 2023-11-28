use crate::db::{establish_connection, DbPool};
use crate::handlers::account::account_scope;
use crate::handlers::files::files_scope;
use crate::handlers::invites::invite_scope;
use crate::handlers::upload::upload_scope;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::{info, warn};
use std::{env, thread};

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
    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection_attempts = 0;
    let max_attempts = env::var("MAX_ATTEMPTS")
        .expect("MAX_ATTEMPTS must be set")
        .parse::<u32>()
        .expect("MAX_ATTEMPTS must be a valid integer");
    let retry_delay = std::time::Duration::from_secs(
        env::var("RETRY_DELAY")
            .expect("RETRY_DELAY must be set")
            .parse::<u64>()
            .expect("RETRY_DELAY must be a valid integer"),
    );

    let pool: DbPool = loop {
        match establish_connection(&database_url) {
            Ok(pool) => {
                info!("Successfully connected to database");
                break pool;
            }
            Err(e) => {
                connection_attempts += 1;
                warn!(
                    "Failed to establish database connection: {}. Attempt {}/{}",
                    e, connection_attempts, max_attempts
                );

                if connection_attempts >= max_attempts {
                    panic!(
                        "Failed to establish database connection after {} attempts",
                        max_attempts
                    );
                }

                thread::sleep(retry_delay);
            }
        }
    };

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(account_scope())
            .service(invite_scope())
            .service(upload_scope())
            .service(files_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
