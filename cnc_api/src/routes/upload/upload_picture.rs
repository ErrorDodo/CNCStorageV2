use std::path::PathBuf;

use crate::{
    db::DbPool,
    models::{jwtmodel::Claims, pictures::NewPicture, upload::UploadDTO},
    schema::pictures,
    utils::upload_files::upload_to_azure,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use log::{error, info};
use std::convert::From;

pub async fn upload_image(
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

    info!("Uploading image to Azure Storage");
    let file_url_string =
        match upload_to_azure(&file_data, file_name_str, file_type, username).await {
            Ok(url) => {
                info!("Image uploaded to Azure successfully");
                url
            }
            Err(e) => {
                error!("Error uploading to Azure: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        };

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    if let Some(dto) = upload_dto {
        let new_picture = NewPicture {
            uploaded_by_user_id: claims.sub,
            file_name: file_name_without_extension,
            upload_date: chrono::Utc::now().naive_utc(),
            file_url: file_url_string.clone(),
            file_size: file_data.len() as i64,
            file_format: file_type.to_string(),
            resolution: dto.resolution.clone(),
            tags: Some(dto.tags.iter().map(|tag| Some(tag.clone())).collect()),
        };

        match diesel::insert_into(pictures::table)
            .values(&new_picture)
            .execute(&mut *conn)
        {
            Ok(_) => {
                info!("Successfully inserted picture into database");
                return Ok(HttpResponse::Ok().json({
                    serde_json::json!({
                        "message": "Image uploaded successfully",
                        "url": file_url_string
                    })
                }));
            }
            Err(e) => {
                error!("Error inserting picture into database: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    } else {
        info!("No DTO provided, skipping database insertion");
        return Ok(HttpResponse::Ok().json("Could not insert records into database"));
    }
}
