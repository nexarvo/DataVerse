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

pub async fn create_datasets_table(pool: &PgPool) {
    let query = r#"
        CREATE TABLE datasets (
            id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
            file_name TEXT NOT NULL,
            file_size BIGINT NOT NULL,
            file_type TEXT NOT NULL,
            upload_time TIMESTAMP DEFAULT NOW(),
            uploaded_by UUID, -- Foreign key to a users table if applicable
            dataset_url TEXT NOT NULL,
            row_count INTEGER,
        );
    "#;
    match sqlx::query(query).execute(pool).await {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Error creating table: {:?}", e),
    }
}

pub async fn create_transformations_table(pool: &PgPool) {
    let query = r#"
        CREATE TABLE transformations (
            id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
            dataset_id UUID NOT NULL REFERENCES datasets(id) ON DELETE CASCADE,
            transformation_type TEXT NOT NULL, -- e.g., "filter", "aggregation"
            parameters JSONB NOT NULL,
            applied_at TIMESTAMP DEFAULT NOW(),
            applied_by UUID,
            result_preview JSONB
        );
    "#;
    match sqlx::query(query).execute(pool).await {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Error creating table: {:?}", e),
    }
}
