use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

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
