mod db;
mod dto;
mod errors;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;
use crate::db::establish_connection_pool;
use crate::routes::auth::auth_routes;
use crate::routes::dataset::file_routes;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer, Responder,
};
use db::duck_db_migrations::run_duckdb_migrations;
use dotenv::dotenv;
use log::info;
use routes::charts::charts_routes;
use routes::sql_cell::sql_query_routes;
use routes::{cell::cell_routes, dataframe::dataframe_routes};
use sqlx::PgPool;
use std::env;
use std::rc::Rc;
use std::sync::Mutex;
use utils::token_verification_middleware::AuthMiddleware;

async fn health_check(pool: Data<PgPool>) -> impl Responder {
    if sqlx::query("SELECT 1")
        .execute(pool.get_ref())
        .await
        .is_ok()
    {
        "Database is connected!"
    } else {
        "Database connection failed!"
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let pool = establish_connection_pool().await;

    // Run migrations
    run_duckdb_migrations();

    info!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            // CORS configuration
            .wrap(
                Cors::default()
                    .allow_any_origin() // For testing purposes, allows any origin. Change this to a specific domain in production.
                    .allowed_origin("http://localhost:5173") // Allow requests from your frontend
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Allowed HTTP methods
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]) // Allowed headers
                    .max_age(3600), // Preflight request cache duration in seconds
            )
            .wrap(AuthMiddleware {
                secret: Rc::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set")),
            })
            .route("/health", web::get().to(health_check))
            .configure(auth_routes)
            .configure(file_routes)
            .configure(dataframe_routes)
            .configure(cell_routes)
            .configure(charts_routes)
            .configure(sql_query_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
