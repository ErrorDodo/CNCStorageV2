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
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
    pub file_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = pictures)]
pub struct NewPicture {
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
    pub file_name: String,
}

#[derive(Serialize)]
pub struct PictureResponse {
    pub file_name: String,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Option<Vec<Option<String>>>,
}
