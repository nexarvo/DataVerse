use std::str::FromStr;

use chrono::Utc;
use log::info;
use polars::frame::DataFrame;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dto::datasets, models::transformation::Transformation};

pub async fn save_transformation_history(
    pool: &PgPool,
    dataset_id: Uuid,
    transformations: Value,
    data_type_str: &str,
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
        result_preview: None,
    };

    let dataset_id = match datasets::DataType::from_str(data_type_str) {
        Ok(datasets::DataType::Dataset) => Some(new_transformation.dataset_id),
        Ok(datasets::DataType::DataFrame) => None,
        Err(err) => {
            info!("Error: {}", err);
            return Err(sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid data type",
            ))));
        }
    };

    sqlx::query!(
        "INSERT INTO transformations (id, dataset_id, transformation_type, parameters) VALUES ($1, $2, $3, $4)",
        new_transformation.id,
        dataset_id,
        new_transformation.transformation_type,
        new_transformation.parameters,
    )
    .execute(pool)
    .await?;

    Ok(new_transformation)
}
