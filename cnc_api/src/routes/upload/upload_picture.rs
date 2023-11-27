use crate::{
    db::DbPool,
    models::{jwtmodel::Claims, pictures::NewPicture, upload::UploadDTO},
    schema::pictures,
};
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use log::info;

pub async fn upload_image(
    upload_dto: &Option<UploadDTO>,
    claims: &Claims,
    file_data: Vec<u8>,
    file_type: &String,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("Uploading image to Azure Storage");
    let file_url_string = upload_to_azure(&file_data, file_type).await?;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // TODO: Testing in Postman cannot get passed this point
    if let Some(dto) = upload_dto {
        let new_picture = NewPicture {
            uploaded_by_user_id: claims.sub,
            upload_date: chrono::Utc::now().naive_utc(),
            file_url: file_url_string,
            file_size: file_data.len() as i64,
            file_format: dto.file_format.clone(),
            resolution: dto.resolution.clone(),
            tags: dto.tags.clone(),
        };
        info!("Inserting picture into database");

        match diesel::insert_into(pictures::table)
            .values(&new_picture)
            .execute(&mut *conn)
        {
            Ok(_) => {
                info!("Picture inserted successfully into database");
                return Ok(HttpResponse::Ok().json("Image uploaded successfully"));
            }
            Err(e) => {
                println!("Error inserting picture into database: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    }

    Ok(HttpResponse::Ok().json("Image uploaded successfully"))
}

// Dummy function for Azure Storage upload - Replace with actual Azure upload logic
async fn upload_to_azure(file_data: &[u8], file_type: &String) -> Result<String, Error> {
    // Implement actual Azure Storage upload logic here
    // Return the URL of the uploaded file
    Ok("https://example.com/uploaded_file".to_string())
}
