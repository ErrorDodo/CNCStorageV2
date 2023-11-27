use crate::models::users::User;
use crate::schema::videos;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(primary_key(video_id))]
#[diesel(belongs_to(User, foreign_key = uploaded_by_user_id))]
#[diesel(table_name = videos)]
pub struct Video {
    pub video_id: Uuid,
    pub file_name: String,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub duration: chrono::Duration,
    pub resolution: String,
    pub tags: Vec<String>,
}
