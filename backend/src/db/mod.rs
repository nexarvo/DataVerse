use sqlx::PgPool;
use std::env;

pub async fn establish_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
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
