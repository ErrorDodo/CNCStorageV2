use crate::models::users::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Associations;
use diesel::Identifiable;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "generated_by_user_id")]
#[table_name = "invites"]
pub struct Invite {
    pub id: Uuid,
    pub generated_by_user_id: Uuid,
    pub has_been_used: bool,
    pub date_used: Option<NaiveDateTime>,
    pub used_by_user_id: Option<Uuid>,
    pub invite_code: String,
}
