use crate::models::users::User;
use crate::schema::event_logs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(primary_key(event_log_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = event_logs)]
pub struct EventLog {
    pub event_log_id: Uuid,
    pub event_type: String,
    pub user_id: Option<Uuid>,
    pub timestamp: NaiveDateTime,
    pub details: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = event_logs)]
pub struct NewEventLog<'a> {
    pub event_type: &'a str,
    pub user_id: Option<Uuid>,
    pub details: Option<&'a str>,
}
