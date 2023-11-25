use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // Subject (user identifier)
    pub exp: usize, // Expiry (as a timestamp)
}
