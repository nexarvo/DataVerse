use chrono::NaiveDateTime;
use serde_json::Value;
use sqlx::{PgPool, Error, Row};
use uuid::Uuid;

use crate::models::datasets::Dataset;

pub async fn insert_new_dataset(
    pool: &PgPool,
    file_name: String,
    file_size: i64,
    file_type: String,
    dataset_url: String,
    upload_time: Option<NaiveDateTime>,
    uploaded_by: Option<Uuid>,
    row_count: Option<i32>,
    latest_preview: Option<Value>
) -> Result<Dataset, Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO datasets (id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        Some(Uuid::new_v4()),
        file_name,
        file_size,
        file_type,
        dataset_url,
        upload_time,
        uploaded_by,
        row_count,
        latest_preview
    )
    .fetch_one(pool) // Fetch the newly inserted row
    .await?;

    // Map the result into a Dataset struct
    let dataset = Dataset {
        id: row.get("id"),
        file_name: row.get("file_name"),
        file_size: row.get("file_size"),
        file_type: row.get("file_type"),
        dataset_url: row.get("dataset_url"),
        upload_time: row.get("upload_time"),
        uploaded_by: row.get("uploaded_by"),
        row_count: row.get("row_count"),
        latest_preview: row.get("latest_preview"),
    };

    // Return the inserted dataset
    Ok(dataset)
}

pub async fn get_datasets(
    pool: &PgPool
) -> Result<Vec<Dataset>, Error> {
    let datasets = sqlx::query_as!(
        Dataset,
        r#"
        SELECT id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview
        FROM datasets
        ORDER BY upload_time DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(datasets)
}

pub async fn get_dataset_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Dataset>, Error> {
    let dataset = sqlx::query_as!(
        Dataset,
        r#"
        SELECT id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview
        FROM datasets
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(dataset)
}