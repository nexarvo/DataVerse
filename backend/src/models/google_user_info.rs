use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,             // Google user ID
    pub email: String,           // User email
    pub name: String,            // User full name
    pub picture: Option<String>, // Profile picture URL (optional)
}
