use std::fs::File;

use crate::services::dataset_service::download_dataset;
use log::{error, info};
use polars::prelude::*;
use polars::io::parquet::ParquetWriter;
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
    dataset_id: Uuid,
    mut df: DataFrame,
    transformations: Vec<Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    info!("Starting apply transformation: {}", dataset_id);

    for transformation in transformations {
        let transform_type = transformation["type"].as_str().ok_or_else(|| {
            Box::<dyn std::error::Error>::from("Missing 'type' in transformation")
        })?;
        
        match transform_type {
            "filter" => {
                let column = transformation["params"]["column"].as_str().ok_or_else(|| {
                    Box::<dyn std::error::Error>::from("Missing 'column' in transformation")
                })?;
                let value = &transformation["params"]["value"];
                let operation = transformation["params"]["operation"]
                    .as_str()
                    .ok_or_else(|| {
                        Box::<dyn std::error::Error>::from(
                            "Missing 'operation' in filter transformation",
                        )
                    })?;

                // Determine column type
                let column_series = df.column(column)?;
                match column_series.dtype() {
                    DataType::Utf8 => {
                        info!("Came Utf8");
                        let value_str = value.as_str().ok_or_else(|| {
                            Box::<dyn std::error::Error>::from("Value type mismatch for Utf8 column")
                        })?;
                        let column_data = column_series.utf8()?;

                        let mask = column_data
                            .into_iter()
                            .map(|opt| match operation {
                                ">" => opt.map(|s| s > value_str).unwrap_or(false),
                                "<" => opt.map(|s| s < value_str).unwrap_or(false),
                                ">=" => opt.map(|s| s >= value_str).unwrap_or(false),
                                "<=" => opt.map(|s| s <= value_str).unwrap_or(false),
                                "==" => opt.map(|s| s == value_str).unwrap_or(false),
                                _ => false,
                            })
                            .collect::<BooleanChunked>();

                        df = df.filter(&mask)?;
                    }
                    DataType::Int64 => {
                        let value_int = value.as_i64().ok_or_else(|| {
                            Box::<dyn std::error::Error>::from("Value type mismatch for Int64 column")
                        })?;
                        let column_data = column_series.i64()?;
                        info!("Came Int64");

                        let mask = column_data
                            .into_iter()
                            .map(|opt| match operation {
                                "Greater than" => opt.map(|v| v > value_int).unwrap_or(false),
                                "<" => opt.map(|v| v < value_int).unwrap_or(false),
                                ">=" => opt.map(|v| v >= value_int).unwrap_or(false),
                                "<=" => opt.map(|v| v <= value_int).unwrap_or(false),
                                "==" => opt.map(|v| v == value_int).unwrap_or(false),
                                _ => false,
                            })
                            .collect::<BooleanChunked>();

                        df = df.filter(&mask)?;
                    }
                    DataType::Float64 => {
                        info!("Came Float64");
                    
                        // Assuming `value` is a `&JsonValue` (e.g., `&JsonValue` might represent a JSON field)
                        let value_str = value.as_str().ok_or_else(|| {
                            Box::<dyn std::error::Error>::from("Expected a string value for Float64 column")
                        })?;
                    
                        let value_f64 = value_str
                            .parse::<f64>()
                            .map_err(|_| Box::<dyn std::error::Error>::from("Value type mismatch for Float64 column: unable to parse string to f64"))?;
                    
                        let column_data = column_series.f64()?;
                    
                        // Perform the filtering based on the parsed f64 value
                        let mask = column_data
                            .into_iter()
                            .map(|opt| match operation {
                                "Greater than" => opt.map(|v| v > value_f64).unwrap_or(false),
                                "<" => opt.map(|v| v < value_f64).unwrap_or(false),
                                ">=" => opt.map(|v| v >= value_f64).unwrap_or(false),
                                "<=" => opt.map(|v| v <= value_f64).unwrap_or(false),
                                "==" => opt.map(|v| (v - value_f64).abs() < f64::EPSILON).unwrap_or(false),
                                _ => false,
                            })
                            .collect::<BooleanChunked>();
                    
                        df = df.filter(&mask)?;
                    }
                    _ => {
                        return Err(Box::from("Unsupported column type for filtering"));
                    }
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

    // Step 3: Save DataFrame as a Parquet file
    let file_path = format!("/tmp/{}.parquet", dataset_id);
    save_as_parquet(df, &file_path)?;

    info!("Successfully applied transformations: {}", dataset_id);
    Ok(file_path)
}

pub fn save_as_parquet(mut df: DataFrame, file_path: &str) -> Result<(), PolarsError> {
    let file = File::create(file_path)?;
    ParquetWriter::new(file).finish(&mut df)?;
    Ok(())
}
