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

impl<'a> NewUser<'a> {
    pub fn new(
        username: &'a str,
        password_hash: &'a str,
        date_registered: NaiveDateTime,
        invited_by_user_id: Option<Uuid>,
        is_admin: bool,
        is_moderator: bool,
    ) -> Self {
        NewUser {
            username,
            password_hash,
            date_registered,
            invited_by_user_id,
            is_admin: false,
            is_moderator: false,
        }
    }
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
