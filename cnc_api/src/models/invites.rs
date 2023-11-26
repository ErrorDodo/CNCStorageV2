use crate::models::users::User;
use crate::schema::invites;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(table_name = invites)]
#[diesel(primary_key(invite_id))]
#[diesel(belongs_to(User, foreign_key = generated_by_user_id))]
pub struct Invite {
    pub invite_id: Uuid,
    pub generated_by_user_id: Uuid,
    pub has_been_used: bool,
    pub date_used: Option<NaiveDateTime>,
    pub used_by_user_id: Option<Uuid>,
    pub invite_code: String,
}

#[derive(Insertable)]
#[diesel(table_name = invites)]
pub struct NewInvite<'a> {
    pub generated_by_user_id: Uuid,
    pub has_been_used: bool,
    pub date_used: Option<NaiveDateTime>,
    pub used_by_user_id: Option<Uuid>,
    pub invite_code: &'a str,
}

#[derive(Deserialize)]
pub struct CreateInviteDTO {
    pub username: String,
}
