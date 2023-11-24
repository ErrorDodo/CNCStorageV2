use crate::models::users::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Associations;
use diesel::Identifiable;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "uploaded_by_user_id")]
#[table_name = "pictures"]
pub struct Picture {
    pub id: Uuid,
    pub uploaded_by_user_id: Uuid,
    pub upload_date: NaiveDateTime,
    pub file_url: String,
    pub file_size: i64,
    pub file_format: String,
    pub resolution: String,
    pub tags: Vec<String>,
}
