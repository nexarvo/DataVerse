use sqlx::PgPool;
use std::env;

pub async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn create_user_table(pool: &PgPool) {
    let query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash TEXT,
            google_id TEXT UNIQUE
        )
    "#;
    sqlx::query(query).execute(pool).await.unwrap();
}
