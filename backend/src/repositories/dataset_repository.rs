use chrono::NaiveDateTime;
use serde_json::Value;
use sqlx::{Error, PgPool, Row};
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
    latest_preview: Option<Value>,
    column_metadata: Option<Value>,
) -> Result<Dataset, Error> {
    // Insert the dataset into the database and return the number of affected rows
    let query_result = sqlx::query!(
        r#"
        INSERT INTO datasets (id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview, column_metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview, column_metadata
        "#,
        Some(Uuid::new_v4()), // Generate new UUID for the `id`
        file_name,
        file_size,
        file_type,
        dataset_url,
        upload_time,
        uploaded_by,
        row_count,
        latest_preview,
        column_metadata
    )
    .fetch_one(pool) // Fetch the row returned by `RETURNING`
    .await?;

    // Map the result into a Dataset struct
    let dataset = Dataset {
        id: Some(query_result.id),
        file_name: query_result.file_name,
        file_size: query_result.file_size,
        file_type: query_result.file_type,
        dataset_url: query_result.dataset_url,
        upload_time: query_result.upload_time,
        uploaded_by: query_result.uploaded_by,
        row_count: query_result.row_count,
        column_metadata: query_result.column_metadata,
        latest_preview: query_result.latest_preview,
    };

    // Return the inserted dataset
    Ok(dataset)
}

pub async fn get_datasets(pool: &PgPool) -> Result<Vec<Dataset>, Error> {
    let datasets = sqlx::query_as!(
        Dataset,
        r#"
        SELECT id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview, column_metadata
        FROM datasets
        ORDER BY upload_time DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(datasets)
}

pub async fn get_dataset_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Dataset>, Error> {
    let dataset = sqlx::query_as!(
        Dataset,
        r#"
        SELECT id, file_name, file_size, file_type, dataset_url, upload_time, uploaded_by, row_count, latest_preview, column_metadata
        FROM datasets
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(dataset)
}
