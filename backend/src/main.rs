use actix_web::{middleware, web, App, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use std::env;

async fn health_check() -> impl Responder {
    "Backend is running!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    info!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
