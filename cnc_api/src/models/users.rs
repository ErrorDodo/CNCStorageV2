use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub auth_token: String,
    pub date_registered: NaiveDateTime,
    pub invited_by_user_id: Option<Uuid>,
    pub is_admin: bool,
    pub is_moderator: bool,
}
