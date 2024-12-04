use crate::services::file_service::download_dataset;
use log::{error, info};
use polars::prelude::*;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn load_dataset(
    dataset_id: Uuid,
    pool: &sqlx::PgPool,
) -> Result<polars::prelude::DataFrame, Box<dyn std::error::Error>> {
    info!("Starting to load dataset for dataset_id: {}", dataset_id);
    let query = "SELECT dataset_url FROM datasets WHERE id = $1";
    let dataset_url: String = sqlx::query_scalar(query)
        .bind(dataset_id)
        .fetch_one(pool)
        .await?;

    let file_path = download_dataset(&dataset_url).await?;
    let df = polars::prelude::CsvReader::from_path(file_path.clone())?.finish()?;

    tokio::fs::remove_file(file_path).await?;

    Ok(df)
}

pub fn apply_transformations(
    mut df: DataFrame,
    transformations: Vec<Value>,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // Log transformations for debugging
    for transformation in transformations {
        // Safely extract the 'type' field from each transformation
        let transform_type = transformation["type"].as_str().ok_or_else(|| {
            Box::<dyn std::error::Error>::from("Missing 'type' in transformation")
        })?;
        match transform_type {
            "filter" => {
                let column = transformation["params"]["column"].as_str().ok_or_else(|| {
                    Box::<dyn std::error::Error>::from("Missing 'column' in transformation")
                })?;
                let value = &transformation["params"]["value"];

                // Extract the operation and apply it accordingly
                let operation =
                    transformation["params"]["operation"]
                        .as_str()
                        .ok_or_else(|| {
                            Box::<dyn std::error::Error>::from(
                                "Missing 'operation' in filter transformation",
                            )
                        })?;

                if value.is_null() {
                    error!(
                        "Filter transformation contains a null value: {:?}",
                        transformation
                    );
                    return Err(Box::from("Filter transformation cannot have null value"));
                } else if let Some(value_str) = value.as_str() {
                    let column_data = df.column(column)?.utf8()?;

                    // Generate the mask based on the operation
                    let mask = column_data
                        .into_iter()
                        .map(|opt| {
                            match operation {
                                ">" => opt.map(|s| s > value_str).unwrap_or(false),
                                "<" => opt.map(|s| s < value_str).unwrap_or(false),
                                ">=" => opt.map(|s| s >= value_str).unwrap_or(false),
                                "<=" => opt.map(|s| s <= value_str).unwrap_or(false),
                                "==" => opt.map(|s| s == value_str).unwrap_or(false),
                                _ => false, // Default case if the operation is not recognized
                            }
                        })
                        .collect::<BooleanChunked>();

                    df = df.filter(&mask)?;
                } else if let Some(value_int) = value.as_i64() {
                    let column_data = df.column(column)?.i64()?;

                    // Generate the mask based on the operation
                    let mask = column_data
                        .into_iter()
                        .map(|opt| {
                            match operation {
                                ">" => opt.map(|v| v > value_int).unwrap_or(false),
                                "<" => opt.map(|v| v < value_int).unwrap_or(false),
                                ">=" => opt.map(|v| v >= value_int).unwrap_or(false),
                                "<=" => opt.map(|v| v <= value_int).unwrap_or(false),
                                "==" => opt.map(|v| v == value_int).unwrap_or(false),
                                _ => false, // Default case if the operation is not recognized
                            }
                        })
                        .collect::<BooleanChunked>();
                    df = df.filter(&mask)?;
                } else if let Some(value_f64) = value.as_f64() {
                    let column_data = df.column(column)?.f64()?;

                    // Generate the mask based on the operation
                    let mask = column_data
                        .into_iter()
                        .map(|opt| {
                            match operation {
                                ">" => opt.map(|v| v > value_f64).unwrap_or(false),
                                "<" => opt.map(|v| v < value_f64).unwrap_or(false),
                                ">=" => opt.map(|v| v >= value_f64).unwrap_or(false),
                                "<=" => opt.map(|v| v <= value_f64).unwrap_or(false),
                                "==" => opt
                                    .map(|v| (v - value_f64).abs() < f64::EPSILON)
                                    .unwrap_or(false), // Apply '=='
                                _ => false, // Default case if the operation is not recognized
                            }
                        })
                        .collect::<BooleanChunked>();

                    df = df.filter(&mask)?;
                } else {
                    error!(
                        "Unsupported value type in filter transformation: {:?}",
                        value
                    );
                    return Err(Box::from("Unsupported value type in filter transformation"));
                }
            }
            "aggregate" => {
                let column = transformation["params"]["column"].as_str().ok_or_else(|| {
                    Box::<dyn std::error::Error>::from("Missing 'column' in aggregation")
                })?;
                match transformation["params"]["operation"].as_str() {
                    Some("sum") => df = df.groupby([column])?.sum()?,
                    Some("mean") => df = df.groupby([column])?.mean()?,
                    Some("count") => df = df.groupby([column])?.count()?,
                    Some("min") => df = df.groupby([column])?.min()?,
                    Some("max") => df = df.groupby([column])?.max()?,
                    _ => {
                        error!("Unsupported aggregation operation: {:?}", transformation);
                        return Err(Box::from("Unsupported aggregation operation"));
                    }
                }
            }
            _ => {
                error!(
                    "Unsupported transformation type: {}",
                    transformation["type"]
                );
                return Err(Box::from("Unsupported transformation type"));
            }
        }
    }

    Ok(df)
}

pub async fn save_transformation_history(
    pool: &PgPool,
    dataset_id: Uuid,
    transformations: &Value,
    transformed_data: &DataFrame,
) -> Result<(), sqlx::Error> {
    // Convert the DataFrame to a JSON-compatible format
    // let result_preview = transformed_data.to_json(None)?.to_string();

    sqlx::query!(
        "INSERT INTO transformations (dataset_id, transformation_type, parameters) VALUES ($1, $2, $3)",
        dataset_id,
        transformations["type"].as_str().unwrap(),
        transformations,
    )
    .execute(pool)
    .await?;

    Ok(())
}
