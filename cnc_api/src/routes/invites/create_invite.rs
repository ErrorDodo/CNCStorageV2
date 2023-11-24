use actix_web::HttpResponse;

pub async fn build_invite() -> HttpResponse {
    HttpResponse::Ok().body("build_invite")
}
