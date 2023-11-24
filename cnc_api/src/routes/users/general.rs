use crate::routes::users::add_user::create_user;
use actix_web::{http::Method, HttpRequest, HttpResponse, Responder};
use log::info;

pub async fn handle_user_actions(req: HttpRequest) -> impl Responder {
    info!(
        "handle_user_actions called with method: {}, path: {}",
        req.method(),
        req.path()
    );

    match req.path() {
        "/users/add" if req.method() == Method::POST => create_user().await,
        // Add other paths and methods here, e.g., "/users/remove", "/users/list", etc.
        // "/users/remove" if req.method() == Method::DELETE => remove_user().await,
        // "/users/list" if req.method() == Method::GET => list_users().await,
        _ => HttpResponse::MethodNotAllowed().body("Method Not Allowed or Path Not Recognized"),
    }
}
