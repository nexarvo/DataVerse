use std::fs::File;
use std::str::FromStr;

use crate::dto::datasets;
use crate::services::dataframe_service::{
    download_parquet_from_supabase, read_parquet_to_dataframe,
};
use crate::services::dataset_service::download_dataset;
use chrono::Datelike;
use chrono::NaiveDate;
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
                df = apply_filter_transformation(df, &transformation)?;
            }
            "aggregate" => {
                df = apply_aggregate_transformation(df, &transformation)?;
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

    // Save DataFrame as a Parquet file
    let file_path = format!("/tmp/{}.parquet", dataset_id);
    save_as_parquet(df, &file_path)?;

    info!("Successfully applied transformations: {}", dataset_id);
    Ok(file_path)
}

fn apply_filter_transformation(
    mut df: DataFrame,
    transformation: &Value,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let column = transformation["params"]["column"].as_str().ok_or_else(|| {
        Box::<dyn std::error::Error>::from("Missing 'column' in filter transformation")
    })?;
    let value = &transformation["params"]["value"];
    let operation = transformation["params"]["operation"]
        .as_str()
        .ok_or_else(|| {
            Box::<dyn std::error::Error>::from("Missing 'operation' in filter transformation")
        })?;

    // Determine column type and apply the filter
    let column_series = df.column(column)?;
    match column_series.dtype() {
        DataType::Utf8 => {
            df = filter_string_column(df, column, value, operation)?;
        }
        DataType::Int64 | DataType::Float64 => {
            df = filter_numeric_column(df, column, value, operation)?;
        }
        DataType::Date => {
            df = filter_date_column(df, column, value, operation)?;
        }
        _ => {
            return Err(Box::from("Unsupported column type for filtering"));
        }
    }

    Ok(df)
}

fn filter_string_column(
    mut df: DataFrame,
    column: &str,
    value: &Value,
    operation: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let value_str = value.as_str().ok_or_else(|| {
        Box::<dyn std::error::Error>::from("Expected a string value for string column")
    })?;
    let column_series = df.column(column)?.utf8()?;

    let mask = column_series
        .into_iter()
        .map(|opt| match operation {
            "Contains" => opt.map(|s| s.contains(value_str)).unwrap_or(false),
            "Does not contain" => opt.map(|s| !s.contains(value_str)).unwrap_or(false),
            "Is equal to" => opt.map(|s| s == value_str).unwrap_or(false),
            "Is not equal to" => opt.map(|s| s != value_str).unwrap_or(false),
            "Starts with" => opt.map(|s| s.starts_with(value_str)).unwrap_or(false),
            "Does not start with" => opt.map(|s| !s.starts_with(value_str)).unwrap_or(false),
            "Ends with" => opt.map(|s| s.ends_with(value_str)).unwrap_or(false),
            "Does not end with" => opt.map(|s| !s.ends_with(value_str)).unwrap_or(false),
            "Is null" => opt.is_none(),
            "Is not null" => opt.is_some(),
            _ => false,
        })
        .collect::<BooleanChunked>();

    df = df.filter(&mask)?;
    Ok(df)
}

fn filter_numeric_column(
    mut df: DataFrame,
    column: &str,
    value: &Value,
    operation: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let value_str = value.as_str().ok_or_else(|| {
        Box::<dyn std::error::Error>::from("Expected a string value for numeric column")
    })?;

    let value_parsed = if *df.column(column)?.dtype() == DataType::Int64 {
        value_str
            .parse::<i64>()
            .map_err(|_| Box::<dyn std::error::Error>::from("Failed to parse value to Int64"))?
            as f64 // Convert i64 to f64
    } else {
        value_str
            .parse::<f64>()
            .map_err(|_| Box::<dyn std::error::Error>::from("Failed to parse value to Float64"))?
    };

    let column_series: Series = if *df.column(column)?.dtype() == DataType::Int64 {
        df.column(column)?.clone() // Clone the column to ensure a common type
    } else {
        df.column(column)?.clone() // Clone the column to ensure a common type
    };

    let mask = match column_series.dtype() {
        DataType::Int64 => {
            let column_data = column_series.i64()?;
            column_data
                .into_iter()
                .map(|opt| match operation {
                    "Is equal to" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v == value_parsed as i64).unwrap_or(false)
                    }
                    "Is not equal to" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v != value_parsed as i64).unwrap_or(false)
                    }
                    "Is null" => opt.is_none(),
                    "Is not null" => opt.is_some(),
                    "Greater than" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v > value_parsed as i64).unwrap_or(false)
                    }
                    "Greater than or equal to" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v >= value_parsed as i64).unwrap_or(false)
                    }
                    "Less than" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v < value_parsed as i64).unwrap_or(false)
                    }
                    "Less than or equal to" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v <= value_parsed as i64).unwrap_or(false)
                    }
                    "Is one of" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v == value_parsed as i64).unwrap_or(false)
                    }
                    "Is not one of" => {
                        // Ensure value_parsed is of type i64
                        opt.map(|v| v != value_parsed as i64).unwrap_or(false)
                    }
                    _ => false,
                })
                .collect::<BooleanChunked>()
        }
        DataType::Float64 => {
            let column_data = column_series.f64()?;
            column_data
                .into_iter()
                .map(|opt| match operation {
                    "Is equal to" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v == value_parsed as f64).unwrap_or(false)
                    }
                    "Is not equal to" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v != value_parsed as f64).unwrap_or(false)
                    }
                    "Is null" => opt.is_none(),
                    "Is not null" => opt.is_some(),
                    "Greater than" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v > value_parsed as f64).unwrap_or(false)
                    }
                    "Greater than or equal to" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v >= value_parsed as f64).unwrap_or(false)
                    }
                    "Less than" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v < value_parsed as f64).unwrap_or(false)
                    }
                    "Less than or equal to" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v <= value_parsed as f64).unwrap_or(false)
                    }
                    "Is one of" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v == value_parsed as f64).unwrap_or(false)
                    }
                    "Is not one of" => {
                        // Ensure value_parsed is of type f64
                        opt.map(|v| v != value_parsed as f64).unwrap_or(false)
                    }
                    _ => false,
                })
                .collect::<BooleanChunked>()
        }
        _ => {
            return Err(Box::from("Unsupported column type for filtering"));
        }
    };

    df = df.filter(&mask)?;
    Ok(df)
}

fn filter_date_column(
    mut df: DataFrame,
    column: &str,
    value: &Value,
    operation: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let value_str = value.as_str().ok_or_else(|| {
        Box::<dyn std::error::Error>::from("Expected a string value for Date column")
    })?;

    let value_date = NaiveDate::parse_from_str(value_str, "%Y-%m-%d")
        .map_err(|_| Box::<dyn std::error::Error>::from("Failed to parse value to Date"))?;

    let column_series = df.column(column)?;

    match column_series.dtype() {
        DataType::Date => {
            let column_data = column_series.date()?;
            let mask = column_data
                .into_iter()
                .map(|opt| match operation {
                    "Is on" => opt
                        .map(|ts| {
                            // Convert Unix timestamp to NaiveDate
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date == Some(value_date)
                        })
                        .unwrap_or(false),
                    "Is not on" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date != Some(value_date)
                        })
                        .unwrap_or(false),

                    "Is before" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date < Some(value_date)
                        })
                        .unwrap_or(false),

                    "Is after" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date > Some(value_date)
                        })
                        .unwrap_or(false),

                    "Is on or before" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date <= Some(value_date)
                        })
                        .unwrap_or(false),

                    "Is on or after" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date >= Some(value_date)
                        })
                        .unwrap_or(false),
                    //TODO: will handle this in the future
                    // "Is between" => {
                    //     let start_date = value_str.split("to").next().unwrap();
                    //     let end_date = value_str.split("to").last().unwrap();
                    //     let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
                    //     let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;
                    //     let start_days = start_date.num_days_from_ce();
                    //     let end_days = end_date.num_days_from_ce();

                    //     opt.map(|ts| {
                    //         let ts_date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                    //         ts_date >= Some(start_date) && ts_date <= Some(end_date)
                    //     })
                    //     .unwrap_or(false)
                    // }
                    // "Is not between" => {
                    //     let start_date = value_str.split("to").next().unwrap();
                    //     let end_date = value_str.split("to").last().unwrap();
                    //     let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
                    //     let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;
                    //     let start_days = start_date.num_days_from_ce();
                    //     let end_days = end_date.num_days_from_ce();

                    //     opt.map(|ts| {
                    //         let ts_date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                    //         ts_date < Some(start_date) || ts_date > Some(end_date)
                    //     })
                    //     .unwrap_or(false)
                    // }
                    "Is null" => opt.is_none(),
                    "Is not null" => opt.is_some(),
                    _ => false,
                })
                .collect::<BooleanChunked>();

            df = df.filter(&mask)?;
        }
        DataType::Int32 => {
            // If the column is of type Int32 (e.g., Unix timestamp), we need to convert to NaiveDate
            let column_data = column_series.i32()?;
            let mask = column_data
                .into_iter()
                .map(|opt| match operation {
                    "Is on" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date == Some(value_date)
                        })
                        .unwrap_or(false),
                    "Is not on" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date != Some(value_date)
                        })
                        .unwrap_or(false),
                    "Is before" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date < Some(value_date)
                        })
                        .unwrap_or(false),
                    "Is after" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date > Some(value_date)
                        })
                        .unwrap_or(false),
                    "Is on or before" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date <= Some(value_date)
                        })
                        .unwrap_or(false),
                    "Is on or after" => opt
                        .map(|ts| {
                            let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                            date >= Some(value_date)
                        })
                        .unwrap_or(false),
                    //TODO: will handle below in the future
                    // "Is between" => {
                    //     let start_date = value_str.split("to").next().unwrap();
                    //     let end_date = value_str.split("to").last().unwrap();
                    //     let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
                    //     let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;
                    //     opt.map(|ts| {
                    //         let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                    //         date >= Some(start_date) && date <= Some(end_date)
                    //     })
                    //     .unwrap_or(false)
                    // }
                    // "Is not between" => {
                    //     let start_date = value_str.split("to").next().unwrap();
                    //     let end_date = value_str.split("to").last().unwrap();
                    //     let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
                    //     let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;
                    //     opt.map(|ts| {
                    //         let date = NaiveDate::from_num_days_from_ce_opt(ts as i32);
                    //         date < Some(start_date) || date > Some(end_date)
                    //     })
                    //     .unwrap_or(false)
                    // }
                    "Is null" => opt.is_none(),
                    "Is not null" => opt.is_some(),
                    _ => false,
                })
                .collect::<BooleanChunked>();

            df = df.filter(&mask)?;
        }
        _ => {
            return Err(Box::from("Unsupported column type for filtering"));
        }
    }

    Ok(df)
}

fn apply_aggregate_transformation(
    mut df: DataFrame,
    transformation: &Value,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let column = transformation["params"]["column"].as_str().ok_or_else(|| {
        Box::<dyn std::error::Error>::from("Missing 'column' in aggregate transformation")
    })?;
    let agg_type = transformation["params"]["aggregation"]
        .as_str()
        .ok_or_else(|| {
            Box::<dyn std::error::Error>::from("Missing 'aggregation' in aggregate transformation")
        })?;

    match agg_type {
        "mean" => {
            df = aggregate_mean(df, column)?;
        }
        "sum" => {
            df = aggregate_sum(df, column)?;
        }
        _ => {
            return Err(Box::from("Unsupported aggregation type"));
        }
    }

    Ok(df)
}

fn aggregate_mean(df: DataFrame, column: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let column_series = df.column(column)?.f64()?;
    let mean_value = column_series.mean();
    info!("Calculated mean: {:?}", mean_value);
    Ok(df)
}

fn aggregate_sum(df: DataFrame, column: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let column_series = df.column(column)?.f64()?;
    let sum_value = column_series.sum();
    info!("Calculated sum: {:?}", sum_value);
    Ok(df)
}

pub fn save_as_parquet(mut df: DataFrame, file_path: &str) -> Result<(), PolarsError> {
    let file = File::create(file_path)?;
    ParquetWriter::new(file).finish(&mut df)?;
    Ok(())
}
