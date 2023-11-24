use crate::routes::invites::create_invite::build_invite;
use actix_web::{http::Method, HttpRequest, HttpResponse, Responder};
use log::info;

pub async fn handle_invite_actions(req: HttpRequest) -> impl Responder {
    info!(
        "handle_invite_actions called with method: {}, path: {}",
        req.method(),
        req.path()
    );

    match req.path() {
        "/invite/create" if req.method() == Method::GET => build_invite().await,
        _ => HttpResponse::MethodNotAllowed().body("Method Not Allowed or Path Not Recognized"),
    }
}
