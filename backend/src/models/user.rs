use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub email: String,
    pub password_hash: String,     // Only for email sign-up
    pub google_id: Option<String>, // For Google Sign-In
}
