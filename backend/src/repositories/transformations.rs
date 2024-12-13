use chrono::Utc;
use polars::frame::DataFrame;
use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;

use crate::models::transformation::Transformation;

pub async fn save_transformation_history(
    pool: &PgPool,
    dataset_id: Uuid,
    transformations: &Value,
) -> Result<Transformation, sqlx::Error> {
    // Convert the DataFrame to a JSON-compatible format
    // let result_preview = transformed_data.to_json(None)?.to_string();

    let new_transformation = Transformation {
        id: Uuid::new_v4(),
        transformation_type: transformations["type"].to_string(),
        parameters: transformations["params"].clone(),
        dataset_id,
        parent_transformation_id: None,
        applied_at: Utc::now(),
        applied_by: None,
        result_preview: None
    };

    sqlx::query!(
        "INSERT INTO transformations (id, dataset_id, transformation_type, parameters) VALUES ($1, $2, $3, $4)",
        new_transformation.id,
        new_transformation.dataset_id,
        new_transformation.transformation_type,
        new_transformation.parameters,
    )
    .execute(pool)
    .await?;

    Ok(new_transformation)
}