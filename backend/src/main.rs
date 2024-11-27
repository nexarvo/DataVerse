mod db;
mod errors;
mod models;
mod routes;
mod services;
mod utils;
use crate::db::create_user_table;
use crate::db::establish_connection;
use crate::routes::auth::auth_routes;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer, Responder,
};
use dotenv::dotenv;
use log::info;
use sqlx::PgPool;
use std::env;

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

    let pool = establish_connection().await;

    create_user_table(&pool).await;

    info!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
            .configure(auth_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
