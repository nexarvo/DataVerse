use crate::models::user::User;
use crate::services::auth_service::{google_sign_in, sign_in, sign_up};
use crate::utils::response::{ErrorResponse, SuccessResponse};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct SignInRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct GoogleSignInRequest {
    id_token: String, // Google ID Token
}

// Sign up route
async fn sign_up_route(pool: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match sign_up(&pool, user.into_inner()).await {
        Ok(token) => HttpResponse::Created().json(SuccessResponse {
            status_code: 201,
            message: "User created successfully".to_string(),
            token: Some(token),
        }),
        Err(err) => {
            let error_response = ErrorResponse {
                error_code: 500,
                message: format!("Failed to create user: {}", err),
            };
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

// Sign in route
async fn sign_in_route(pool: web::Data<PgPool>, creds: web::Json<SignInRequest>) -> impl Responder {
    match sign_in(&pool, creds.email.clone(), creds.password.clone()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

// Google Sign-in route
async fn google_sign_in_route(creds: web::Json<GoogleSignInRequest>) -> impl Responder {
    match google_sign_in(creds.id_token.clone()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

// Register all authentication routes
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/signup", web::post().to(sign_up_route))
        .route("/signin", web::post().to(sign_in_route))
        .route("/google_signin", web::post().to(google_sign_in_route));
}