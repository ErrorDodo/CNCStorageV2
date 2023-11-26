use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,  // Subject (user identifier)
    pub exp: usize, // Expiry (as a timestamp)
}
