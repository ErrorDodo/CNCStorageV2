use crate::{
    db::DbPool,
    models::{
        pictures::{Picture, PictureResponse},
        users::User,
    },
    schema::{pictures, users},
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use log::{error, info};
use serde_json::json;
use uuid::Uuid;

pub async fn get_all_pictures(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let picture_data = pictures::table
        .inner_join(users::table.on(pictures::uploaded_by_user_id.eq(users::user_id)))
        .select((pictures::all_columns, users::username))
        .order(pictures::upload_date.desc())
        .load::<(Picture, String)>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load pictures: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to load pictures")
        })?;

    let picture_list: Vec<PictureResponse> = picture_data
        .into_iter()
        .map(|(pic, username)| PictureResponse {
            uploaded_by_username: username,
            file_name: pic.file_name,
            uploaded_by_user_id: pic.uploaded_by_user_id,
            upload_date: pic.upload_date,
            file_url: pic.file_url,
            file_size: pic.file_size,
            file_format: pic.file_format,
            resolution: pic.resolution,
            tags: pic.tags.map(|ts| ts.into_iter().map(|tag| tag).collect()),
        })
        .collect();

    Ok(HttpResponse::Ok().json(picture_list))
}

pub async fn get_pictures_by_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user_uuid = user_id.into_inner();

    let picture_data = pictures::table
        .filter(pictures::uploaded_by_user_id.eq(user_uuid))
        .order(pictures::upload_date.desc())
        .load::<Picture>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load pictures for user {}: {}", user_uuid, e);
            actix_web::error::ErrorInternalServerError("Failed to load pictures")
        })?;

    let picture_responses: Vec<_> = picture_data.into_iter().map(|pic| {
        json!({
            "file_name": pic.file_name,
            "uploaded_by_user_id": pic.uploaded_by_user_id,
            "upload_date": pic.upload_date,
            "file_url": pic.file_url,
            "file_size": pic.file_size,
            "file_format": pic.file_format,
            "resolution": pic.resolution,
            "tags": pic.tags.unwrap_or_default().iter().filter_map(|t| t.as_ref()).cloned().collect::<Vec<String>>(),
        })
    }).collect();

    Ok(HttpResponse::Ok().json(picture_responses))
}

pub async fn recent_upload_pictures(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let picture_data = pictures::table
        .inner_join(users::table.on(pictures::uploaded_by_user_id.eq(users::user_id)))
        .select((pictures::all_columns, users::username))
        .order(pictures::upload_date.desc())
        .limit(10)
        .load::<(Picture, String)>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load recent pictures: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to load recent pictures")
        })?;

    let picture_list: Vec<PictureResponse> = picture_data
        .into_iter()
        .map(|(pic, username)| PictureResponse {
            uploaded_by_username: username,
            file_name: pic.file_name,
            uploaded_by_user_id: pic.uploaded_by_user_id,
            upload_date: pic.upload_date,
            file_url: pic.file_url,
            file_size: pic.file_size,
            file_format: pic.file_format,
            resolution: pic.resolution,
            tags: pic.tags.map(|ts| ts.into_iter().map(|tag| tag).collect()),
        })
        .collect();

    Ok(HttpResponse::Ok().json(picture_list))
}
