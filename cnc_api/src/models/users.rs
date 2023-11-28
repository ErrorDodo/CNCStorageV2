use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub date_registered: NaiveDateTime,
    pub invited_by_user_id: Option<Uuid>,
    pub is_admin: bool,
    pub is_moderator: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub date_registered: NaiveDateTime,
    pub invited_by_user_id: Option<Uuid>,
    pub is_admin: bool,
    pub is_moderator: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub invite_code: String,
}

#[derive(Deserialize)]
pub struct UserLoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum CreateUserError {
    UsernameTaken,
    InviteCodeUsed,
    DieselError(DieselError),
    BcryptError(bcrypt::BcryptError),
}
