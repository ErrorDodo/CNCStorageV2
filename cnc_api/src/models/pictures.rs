use crate::models::users::User;
use crate::schema::pictures;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(primary_key(picture_id))]
#[diesel(belongs_to(User, foreign_key = uploaded_by_user_id))]
#[diesel(table_name = pictures)]
pub struct Picture {
    pub picture_id: Uuid,
    pub file_name: String,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Insertable)]
#[diesel(table_name = pictures)]
pub struct NewPicture<'a> {
    pub file_name: &'a str,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: &'a str,
    pub file_size: i64,
    pub file_format: &'a str,
    pub resolution: &'a str,
    pub tags: Option<Vec<&'a str>>,
}

impl<'a> NewPicture<'a> {
    pub fn new(
        file_name: &'a str,
        uploaded_by_user_id: Uuid,
        upload_date: NaiveDateTime,
        file_url: &'a str,
        file_size: i64,
        file_format: &'a str,
        resolution: &'a str,
        tags: Option<Vec<&'a str>>,
    ) -> Self {
        NewPicture {
            file_name,
            uploaded_by_user_id,
            upload_date,
            file_url,
            file_size,
            file_format,
            resolution,
            tags,
        }
    }
}
#[derive(Serialize)]
pub struct PictureResponse {
    pub uploaded_by_username: String,
    pub file_name: String,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
}

// This struct fixes some issues with the diesel query
#[derive(Queryable, Debug)]
pub struct CompletePictureUserData {
    pub picture_id: Uuid,
    pub file_name: String,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
    pub username: String, // Keep this as the last field
}
