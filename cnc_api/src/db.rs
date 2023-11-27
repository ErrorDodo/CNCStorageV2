use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::info;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<DbPool, String> {
    info!("Connecting to {}", obfuscate_db_url(database_url));
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    match r2d2::Pool::builder().build(manager) {
        Ok(pool) => Ok(pool),
        Err(e) => Err(format!("Failed to create pool: {}", e)),
    }
}

fn obfuscate_db_url(url: &str) -> String {
    let mut split = url.split('@');
    let mut obfuscated = String::new();
    if let Some(user_pass) = split.next() {
        let mut user_pass_split = user_pass.split(':');
        if let Some(user) = user_pass_split.next() {
            obfuscated.push_str(user);
            obfuscated.push_str(":********@");
        }
    }
    if let Some(host_port) = split.next() {
        obfuscated.push_str(host_port);
    }
    obfuscated
}
