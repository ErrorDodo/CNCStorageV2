use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use log::error;

use crate::{
    db::DbPool,
    models::videos::{CompleteVideoUserData, VideoResponse},
    schema::{users, videos},
};

pub async fn get_all_videos(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let complete_video_data = videos::table
        .inner_join(users::table.on(videos::uploaded_by_user_id.eq(users::user_id)))
        .select((
            videos::video_id,
            videos::file_name,
            videos::uploaded_by_user_id,
            videos::upload_date,
            videos::file_url,
            videos::file_size,
            videos::file_format,
            videos::duration,
            videos::resolution,
            videos::tags,
            users::username,
        ))
        .order(videos::upload_date.desc())
        .load::<CompleteVideoUserData>(&mut *conn)
        .map_err(|e| {
            error!("Failed to load videos: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to load videos")
        })?;

    let video_list: Vec<VideoResponse> = complete_video_data
        .into_iter()
        .map(|vid_user_data| VideoResponse {
            uploaded_by_username: vid_user_data.username,
            file_name: vid_user_data.file_name,
            uploaded_by_user_id: vid_user_data.uploaded_by_user_id,
            upload_date: vid_user_data.upload_date,
            file_url: vid_user_data.file_url,
            file_size: vid_user_data.file_size,
            file_format: vid_user_data.file_format,
            duration: vid_user_data.duration,
            resolution: vid_user_data.resolution,
            tags: vid_user_data.tags,
        })
        .collect();

    Ok(HttpResponse::Ok().json(video_list))
}
