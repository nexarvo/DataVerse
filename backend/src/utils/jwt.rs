use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub sub: String,
    pub exp: usize,
}

impl Jwt {
    pub fn new(user_id: String) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        Jwt {
            sub: user_id,
            exp: expiration,
        }
    }
}

/// Extracts the user ID from the HttpRequest
/// Returns `Some(user_id)` if present, otherwise `None`
pub fn get_user_id_from_request(req: &HttpRequest) -> Option<Uuid> {
    if let Some(claims) = req.extensions().get::<Jwt>() {
        println!("Claims found: {:?}", claims);  // Log the claims
        return claims.sub.parse::<Uuid>().ok(); // Attempt to parse the String into a Uuid
    }
    println!("No claims found in request extensions.");
    None
}
