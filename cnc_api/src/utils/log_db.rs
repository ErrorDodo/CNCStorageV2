use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

use crate::{db::DbPool, models::eventlogs::NewEventLog, schema::event_logs};

pub async fn log_event(
    pool: web::Data<DbPool>,
    event_type: &str,
    user_id: Option<Uuid>,
    details: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_event = NewEventLog::new(event_type, user_id, Some(details));

    match diesel::insert_into(event_logs::table)
        .values(&new_event)
        .execute(&mut *conn)
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            error!("Error creating event log: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
