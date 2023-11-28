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
    pub duration: i64,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Insertable)]
#[diesel(table_name = videos)]
pub struct NewVideo<'a> {
    pub file_name: &'a str,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: &'a str,
    pub file_size: i64,
    pub file_format: &'a str,
    pub duration: i64,
    pub resolution: &'a str,
    pub tags: Option<Vec<&'a str>>,
}

impl<'a> NewVideo<'a> {
    pub fn new(
        file_name: &'a str,
        uploaded_by_user_id: Uuid,
        upload_date: NaiveDateTime,
        file_url: &'a str,
        file_size: i64,
        file_format: &'a str,
        duration: i64,
        resolution: &'a str,
        tags: Option<Vec<&'a str>>,
    ) -> Self {
        NewVideo {
            file_name,
            uploaded_by_user_id,
            upload_date,
            file_url,
            file_size,
            file_format,
            duration,
            resolution,
            tags,
        }
    }
}
