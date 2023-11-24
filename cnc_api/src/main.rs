use actix_web::{web, App, HttpServer};
mod db;
mod models;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        // .route("/", web::get().to(routes::greet))
        // .route("/users", web::get().to(routes::get_users))
        // ... other routes ...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
