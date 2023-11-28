use crate::{
    db::DbPool,
    models::{
        pictures::{CompletePictureUserData, Picture, PictureResponse},
        users::User,
    },
    schema::{pictures, users},
};
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use log::{error, info};
use serde_json::json;
use uuid::Uuid;

pub async fn get_all_pictures(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let complete_picture_data = pictures::table
        .inner_join(users::table.on(pictures::uploaded_by_user_id.eq(users::user_id)))
        .select((
            pictures::picture_id,
            pictures::file_name,
            pictures::uploaded_by_user_id,
            pictures::upload_date,
            pictures::file_url,
            pictures::file_size,
            pictures::file_format,
            pictures::resolution,
            pictures::tags,
            users::username,
        ))
        .order(pictures::upload_date.desc())
        .load::<CompletePictureUserData>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load pictures: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to load pictures")
        })?;

    let picture_list: Vec<PictureResponse> = complete_picture_data
        .into_iter()
        .map(|pic_user_data| PictureResponse {
            uploaded_by_username: pic_user_data.username,
            file_name: pic_user_data.file_name,
            uploaded_by_user_id: pic_user_data.uploaded_by_user_id,
            upload_date: pic_user_data.upload_date,
            file_url: pic_user_data.file_url,
            file_size: pic_user_data.file_size,
            file_format: pic_user_data.file_format,
            resolution: pic_user_data.resolution,
            tags: pic_user_data.tags,
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
        .inner_join(users::table.on(pictures::uploaded_by_user_id.eq(user_uuid)))
        .select((
            pictures::picture_id,
            pictures::file_name,
            pictures::uploaded_by_user_id,
            pictures::upload_date,
            pictures::file_url,
            pictures::file_size,
            pictures::file_format,
            pictures::resolution,
            pictures::tags,
            users::username,
        ))
        .order(pictures::upload_date.desc())
        .load::<CompletePictureUserData>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load pictures for user {}: {}", user_uuid, e);
            actix_web::error::ErrorInternalServerError("Failed to load pictures")
        })?;

    let picture_responses: Vec<PictureResponse> = picture_data
        .into_iter()
        .map(|pic_user_data| PictureResponse {
            uploaded_by_username: pic_user_data.username,
            file_name: pic_user_data.file_name,
            uploaded_by_user_id: pic_user_data.uploaded_by_user_id,
            upload_date: pic_user_data.upload_date,
            file_url: pic_user_data.file_url,
            file_size: pic_user_data.file_size,
            file_format: pic_user_data.file_format,
            resolution: pic_user_data.resolution,
            tags: pic_user_data.tags,
        })
        .collect();

    Ok(HttpResponse::Ok().json(picture_responses))
}

pub async fn recent_upload_pictures(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let picture_data = pictures::table
        .inner_join(users::table.on(pictures::uploaded_by_user_id.eq(users::user_id)))
        .select((
            pictures::picture_id,
            pictures::file_name,
            pictures::uploaded_by_user_id,
            pictures::upload_date,
            pictures::file_url,
            pictures::file_size,
            pictures::file_format,
            pictures::resolution,
            pictures::tags,
            users::username,
        ))
        .order(pictures::upload_date.desc())
        .limit(10)
        .load::<CompletePictureUserData>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load recent pictures: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to load recent pictures")
        })?;

    let picture_responses: Vec<PictureResponse> = picture_data
        .into_iter()
        .map(|pic_user_data| PictureResponse {
            uploaded_by_username: pic_user_data.username,
            file_name: pic_user_data.file_name,
            uploaded_by_user_id: pic_user_data.uploaded_by_user_id,
            upload_date: pic_user_data.upload_date,
            file_url: pic_user_data.file_url,
            file_size: pic_user_data.file_size,
            file_format: pic_user_data.file_format,
            resolution: pic_user_data.resolution,
            tags: pic_user_data.tags,
        })
        .collect();

    Ok(HttpResponse::Ok().json(picture_responses))
}
