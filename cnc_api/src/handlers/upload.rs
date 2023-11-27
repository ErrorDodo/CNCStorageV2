use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder, Scope};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::{StreamExt, TryStreamExt};
use infer;
use log::{info, warn};

use crate::{
    db::DbPool,
    models::upload::UploadDTO,
    routes::upload::upload_picture::upload_image,
    utils::{errors::auth::AuthError, generate_auth_token::validate_jwt_token},
};

pub fn upload_scope() -> Scope {
    web::scope("/upload").route("/file", web::post().to(handle_upload_file))
}

async fn handle_upload_file(
    auth: Option<BearerAuth>,
    mut payload: Multipart,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match validate_jwt_token(auth).await {
        Ok(claims) => {
            info!("User {} authenticated", claims.sub);
            let mut upload_dto: Option<UploadDTO> = None;
            let mut file_data = Vec::new();

            while let Ok(Some(mut field)) = payload.try_next().await {
                let content_disposition = field.content_disposition();
                let field_name = content_disposition.get_name().unwrap_or("");

                if field_name == "metadata" {
                    let mut data = Vec::new();
                    while let Some(chunk) = field.next().await {
                        match chunk {
                            Ok(data_chunk) => data.extend_from_slice(&data_chunk),
                            Err(e) => {
                                warn!("Error processing metadata: {}", e);
                                return HttpResponse::BadRequest()
                                    .body(format!("Error processing metadata: {}", e));
                            }
                        }
                    }
                    upload_dto = serde_json::from_slice(&data).ok();
                } else if field_name == "file" {
                    while let Some(chunk) = field.next().await {
                        match chunk {
                            Ok(data_chunk) => file_data.extend_from_slice(&data_chunk),
                            Err(e) => {
                                warn!("Error processing file: {}", e);
                                return HttpResponse::BadRequest()
                                    .body(format!("Error processing file: {}", e));
                            }
                        }
                    }

                    let kind = infer::get(&file_data);
                    if let Some(kind) = kind {
                        let file_type_str = kind.mime_type();
                        info!("File type: {}", file_type_str);
                        match file_type_str {
                            "image/png" | "image/jpeg" | "image/gif" => {
                                if let Err(e) = upload_image(
                                    &upload_dto,
                                    &claims,
                                    file_data.clone(),
                                    &file_type_str.to_string(),
                                    pool.clone(),
                                )
                                .await
                                {
                                    return HttpResponse::InternalServerError()
                                        .json(format!("Error uploading image: {}", e));
                                }
                            }
                            "video/mp4" | "video/mpeg" => {
                                // Process as video
                            }
                            _ => return HttpResponse::BadRequest().body("Unsupported file type"),
                        }
                    } else {
                        return HttpResponse::BadRequest().body("Unsupported file type");
                    }
                }
            }

            HttpResponse::Ok().body("File uploaded successfully")
        }
        Err(AuthError::JwtNotFound) => HttpResponse::Unauthorized().body("JWT token not found"),
        Err(AuthError::JwtInvalid) => HttpResponse::Unauthorized().body("Invalid JWT token"),
    }
}
