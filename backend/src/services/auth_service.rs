use actix_web::{HttpRequest, Error};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;
use reqwest::Client;
use std::env;

use crate::models::google_user_info::GoogleUserInfo;
use crate::models::user::User;
use crate::{errors::auth_error::AuthError, utils::jwt::Jwt};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info};
use crate::repositories::user_repository;

use uuid::Uuid;


pub async fn sign_up(pool: &PgPool, user: &User) -> Result<String, AuthError> {
    info!("Starting sign-up process for email: {}", user.email);
    // Check if user already exists 
    let existing_user = user_repository::find_user_by_email(pool, user.email.clone()).await?;

    if let Some(_) = existing_user {
        error!(
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
    let insert_result = user_repository::insert_new_user(pool, user.email.clone(), hashed_password).await;

    match insert_result {
        Ok(_) => info!("Successfully signed up user with email: {}", user.email),
        Err(err) => {
            error!("Database error while inserting user for email {}: {:?}", user.email, err);
            return Err(AuthError::DbError(sqlx::Error::Configuration(Box::new(
                err,
            ))));
        }
    }

    info!("Successfully signed user up with email: {}", user.email);

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

pub async fn sign_in(pool: &PgPool, user_email: String, password: String) -> Result<String, AuthError> {
    info!("Starting sign-in process for email: {}", user_email);

    // Fetch the user
    let user = user_repository::find_user_by_email(pool, user_email.clone()).await?.ok_or(AuthError::InvalidCredentials)?;

    // Ensure the password_hash is not None
    let hashed_password = user.password_hash;

    // Verify password
    if !verify(password, &hashed_password)? {
        return Err(AuthError::InvalidCredentials);
    }

    let secret = env::var("JWT_SECRET")
        .map_err(|_| AuthError::InternalError("JWT_SECRET not set".to_owned()))?;

    // Generate JWT
    let jwt = Jwt::new(user.id.unwrap().to_string());
    let token = encode(
        &Header::default(),
        &jwt,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreationError)?;

    info!("Successfully sign-in for email: {}", user_email);

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

/// Extract the user ID from the JWT token in the request headers.
pub fn extract_user_id_from_token(req: &HttpRequest) -> Result<Uuid, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid token"))?;

    let secret = env::var("JWT_SECRET").map_err(|_| {
        actix_web::error::ErrorInternalServerError("JWT_SECRET not set in environment")
    })?;

    // Decode JWT
    let token_data = decode::<Jwt>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // Parse `sub` as UUID
    let user_id = token_data.claims.sub.parse::<Uuid>().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Invalid user ID in token")
    })?;
    Ok(user_id)
}
