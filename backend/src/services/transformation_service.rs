use std::fs::File;
use std::str::FromStr;

use crate::dto::datasets;
use crate::services::dataframe_service::{
    download_parquet_from_supabase, read_parquet_to_dataframe,
};
use crate::services::dataset_service::download_dataset;
use log::{error, info};
use polars::io::parquet::ParquetWriter;
use polars::prelude::*;
use polars::prelude::{DataFrame, PolarsError};
use serde_json::Value;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

pub async fn load_dataset(
    dataset_id: Uuid,
    dataset_type: &str,
    pool: &sqlx::PgPool,
) -> Result<polars::prelude::DataFrame, Box<dyn std::error::Error>> {
    info!("Starting to load dataset for dataset_id: {}", dataset_id);

    let query = match datasets::DataType::from_str(dataset_type) {
        Ok(datasets::DataType::Dataset) => "SELECT dataset_url FROM datasets WHERE id = $1",
        Ok(datasets::DataType::DataFrame) => {
            "SELECT dataframe_duckdb_reference FROM dataframe WHERE id = $1"
        }
        Err(err) => {
            info!("Error: {}", err);
            return Err(Box::new(actix_web::error::ErrorBadRequest(
                "Invalid data type",
            )));
        }
    };
    let dataset_url: String = sqlx::query_scalar(query)
        .bind(dataset_id)
        .fetch_one(pool)
        .await?;

    let file_path = match datasets::DataType::from_str(dataset_type) {
        Ok(datasets::DataType::Dataset) => download_dataset(&dataset_url, dataset_type).await?,
        Ok(datasets::DataType::DataFrame) => {
            download_parquet_from_supabase(dataset_id, &dataset_url)
                .await?
                .into()
        }
        Err(err) => {
            info!("Error: {}", err);
            return Err(Box::new(actix_web::error::ErrorBadRequest(
                "Invalid data type",
            )));
        }
    };

    let df = match datasets::DataType::from_str(dataset_type) {
        Ok(datasets::DataType::Dataset) => {
            CsvReader::from_path(file_path.clone())
                .and_then(|reader| reader.finish())
                .map_err(|e| anyhow::anyhow!("Failed to read CSV: {}", e)) // Convert to anyhow::Error
        }
        Ok(datasets::DataType::DataFrame) => {
            read_parquet_to_dataframe(file_path.clone())
                .map_err(|e| anyhow::anyhow!("Failed to read Parquet: {}", e)) // Convert to anyhow::Error
        }
        Err(err) => {
            info!("Error: {}", err);
            Err(anyhow::anyhow!("Unsupported dataset type: {}", err)) // Return anyhow::Error
        }
    };

    tokio::fs::remove_file(file_path).await?;

    Ok(df?)
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
                let operation =
                    transformation["params"]["operation"]
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
                        info!("The operation will be on a Utf8 type");
                        let value_str = value.as_str().ok_or_else(|| {
                            Box::<dyn std::error::Error>::from(
                                "Value type mismatch for Utf8 column",
                            )
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
                        info!("The operation will be on a Int64 type");
                        let value_int = value
                            .as_str()
                            .ok_or_else(|| {
                                Box::<dyn std::error::Error>::from(
                                    "Value type mismatch for Int64 column",
                                )
                            })?
                            .parse::<i64>()
                            .map_err(|_| {
                                Box::<dyn std::error::Error>::from("Failed to parse value to Int64")
                            })?;
                        let column_data = column_series.i64()?;

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
                        info!("The operation will be on a Float64 type");

                        // Assuming `value` is a `&JsonValue` (e.g., `&JsonValue` might represent a JSON field)
                        let value_str = value.as_str().ok_or_else(|| {
                            Box::<dyn std::error::Error>::from(
                                "Expected a string value for Float64 column",
                            )
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
                                "==" => opt
                                    .map(|v| (v - value_f64).abs() < f64::EPSILON)
                                    .unwrap_or(false),
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
