use reqwest::Client;
use std::env;

use crate::models::google_user_info::GoogleUserInfo;
use crate::models::user::User;
use crate::{errors::auth_error::AuthError, utils::jwt::Jwt};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info, warn};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn sign_up(pool: &PgPool, user: User) -> Result<String, AuthError> {
    info!("Starting sign-up process for email: {}", user.email);
    // Check if user already exists
    let existing_user = sqlx::query!(r#"SELECT id FROM users WHERE email = $1"#, user.email)
        .fetch_optional(pool)
        .await?;

    if existing_user.is_some() {
        warn!(
            "Attempted sign-up with an already registered email: {}",
            user.email
        );
        return Err(AuthError::UserExists);
    }

    // Hash the password
    let hashed_password = match hash(user.password_hash.as_str(), DEFAULT_COST) {
        Ok(password) => password,
        Err(err) => {
            error!("Error hashing password for email {}: {:?}", user.email, err);
            return Err(AuthError::DbError(sqlx::Error::Configuration(Box::new(
                err,
            ))));
        }
    };

    info!("Password hashed successfully for email: {}", user.email);

    // Insert new user
    match sqlx::query!(
        r#"INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)"#,
        Uuid::new_v4(),
        user.email,
        hashed_password
    )
    .execute(pool)
    .await
    {
        Ok(_) => info!("Successfully signed user up with email: {}", user.email),
        Err(err) => {
            error!(
                "Database error while inserting new user for email {}: {:?}",
                user.email, err
            );
            return Err(AuthError::DbError(err));
        }
    };

    // Generate JWT token
    let secret = env::var("JWT_SECRET")
        .map_err(|_| AuthError::InternalError("JWT_SECRET not set".to_string()))?;
    let jwt = Jwt {
        sub: user.email.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &jwt,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| AuthError::InternalError(format!("Error encoding JWT: {}", err)))?;

    Ok(token)
}

pub async fn sign_in(pool: &PgPool, email: String, password: String) -> Result<String, AuthError> {
    info!("Starting sign-in process for email: {}", email);

    // Fetch the user
    let user = sqlx::query!(
        r#"SELECT id, password_hash FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AuthError::InvalidCredentials)?;

    // Ensure the password_hash is not None
    let hashed_password = user.password_hash.ok_or(AuthError::InvalidCredentials)?;

    // Verify password
    if !verify(password, &hashed_password)? {
        return Err(AuthError::InvalidCredentials);
    }

    let secret = env::var("JWT_SECRET")
        .map_err(|_| AuthError::InternalError("JWT_SECRET not set".to_owned()))?;

    // Generate JWT
    let jwt = Jwt::new(user.id.to_string());
    let token = encode(
        &Header::default(),
        &jwt,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreationError)?;

    info!("Successfully sign-in for email: {}", email);

    Ok(token)
}

pub async fn google_sign_in(id_token: String) -> Result<String, AuthError> {
    let client = Client::new();

    // Send request to Google's OAuth 2.0 token info endpoint to verify the ID token
    let response = client
        .get(format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            id_token
        ))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // Parse the Google user information
                let google_user: GoogleUserInfo = res.json().await.unwrap();
                let user_email = google_user.email;

                // You can now use the email to create or find a user in your database
                // For the sake of simplicity, we assume the user exists or is created

                // Generate a JWT token for the user
                let secret =
                    env::var("JWT_SECRET").unwrap_or_else(|_| "your_jwt_secret".to_string());

                // Create JWT claims with the user's email
                let jwt = Jwt {
                    sub: user_email.clone(),
                    exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
                };

                // Encode the claims and return the JWT
                let token = encode(
                    &Header::default(),
                    &jwt,
                    &EncodingKey::from_secret(secret.as_bytes()),
                )
                .map_err(|_| AuthError::InternalError("Failed to generate JWT".to_owned()))?;

                Ok(token)
            } else {
                Err(AuthError::Unauthorized)
            }
        }
        Err(_) => Err(AuthError::InternalError(
            "Failed to verify Google token".to_owned(),
        )),
    }
}
