use actix_web::HttpResponse;

pub async fn create_user() -> HttpResponse {
    HttpResponse::Ok().body("add_user")
}
