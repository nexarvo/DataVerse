use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer, Responder,
};
use dotenv::dotenv;
use log::info;
use sqlx::{postgres::PgPoolOptions, PgPool};
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    info!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
