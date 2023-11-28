use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use log::{error, info};
use std::path::PathBuf;

use crate::{
    db::DbPool,
    models::{jwtmodel::Claims, upload::UploadDTO, videos::NewVideo},
    schema::videos,
    utils::upload_files::upload_to_azure,
};

pub async fn upload_video(
    upload_dto: &Option<UploadDTO>,
    claims: &Claims,
    file_data: Vec<u8>,
    file_type: &String,
    file_name: Option<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("Starting upload process");

    let file_name_str = file_name.unwrap_or_else(|| "default_name".to_string());
    let file_name_without_extension = PathBuf::from(file_name_str.clone())
        .with_extension("")
        .to_string_lossy()
        .to_string();

    let username = claims.username.clone();

    info!("Uploading video to Azure Storage");
    let file_url_string =
        match upload_to_azure(&file_data, file_name_str, file_type, username).await {
            Ok(url) => {
                info!("Video uploaded to Azure successfully");
                url
            }
            Err(e) => {
                error!("Error uploading to Azure: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        };

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    if let Some(dto) = upload_dto {
        let new_video = NewVideo::new(
            &file_name_without_extension,
            claims.sub,
            chrono::Utc::now().naive_utc(),
            &file_url_string,
            file_data.len() as i64,
            file_type,
            dto.duration.unwrap_or(0),
            &dto.resolution,
            Some(dto.tags.iter().map(|tag| tag.as_str()).collect()),
        );

        match diesel::insert_into(videos::table)
            .values(&new_video)
            .execute(&mut conn)
        {
            Ok(_) => {
                info!("Video uploaded to database successfully");
                return Ok(HttpResponse::Ok().json({
                    serde_json::json!({
                        "message": "Video uploaded successfully",
                        "url": file_url_string
                    })
                }));
            }
            Err(e) => {
                error!("Error uploading video to database: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    } else {
        return Ok(HttpResponse::InternalServerError().finish());
    }
}
