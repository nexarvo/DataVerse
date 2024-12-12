use sqlx::PgPool;
use std::env;

pub mod duck_db_connection;
pub mod duck_db_migrations;

pub async fn establish_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
