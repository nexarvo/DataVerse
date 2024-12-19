use std::path::PathBuf;

use crate::{
    dto::charts::{ChartDataResponse, ChartRequest},
    repositories::{dataframe::get_dataframe_by_id, dataset_repository::get_dataset_by_id},
    services::{
        dataframe_service::{download_parquet_from_supabase, read_parquet_to_dataframe},
        dataset_service::download_dataset,
    },
};
use actix_web::{web, HttpResponse};
use log::info;
use polars::prelude::*;
use sqlx::PgPool;

async fn generate_chart(
    pool: web::Data<PgPool>,
    data: web::Json<ChartRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Fetch dataset by ID (e.g., from Supabase)
    let dataset_id = data.dataset_id;
    let file_path = match data.is_dataset {
        true => {
            let dataset = get_dataset_by_id(&pool, dataset_id).await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to retrieve dataset: {}",
                    e
                ))
            })?;

            // Check if the dataset is Some or None
            let dataset_url = match dataset {
                Some(ds) => ds.dataset_url, // Access dataset_url here
                None => return Err(actix_web::error::ErrorNotFound("Dataset not found")), // Handle None case
            };

            // Proceed with downloading the dataset
            let dataset_path = download_dataset(&dataset_url).await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to download dataset: {}",
                    e
                ))
            })?;
            dataset_path
        }
        false => {
            let dataframe = get_dataframe_by_id(&pool, dataset_id).await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to retrieve dataframe: {}",
                    e
                ))
            })?;
            let dataframe_reference = match dataframe {
                Some(df) => df.dataframe_duckdb_reference,
                None => return Err(actix_web::error::ErrorNotFound("Dataframe not found")),
            };
            let result = download_parquet_from_supabase(dataset_id, &dataframe_reference)
                .await
                .map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!(
                        "Failed to download dataframe: {}",
                        e
                    ))
                })?;

            PathBuf::from(result)
        }
    };
    let df_result = if data.is_dataset {
        CsvReader::from_path(file_path.clone())
            .and_then(|reader| reader.finish())
            .map_err(|e| anyhow::anyhow!("Failed to read CSV: {}", e))
    } else {
        read_parquet_to_dataframe(file_path.clone())
            .map_err(|e| anyhow::anyhow!("Failed to read Parquet: {}", e))
    };

    let df = match df_result {
        Ok(df) => df,
        Err(err) => {
            info!("Error: {}", err);
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Failed to read dataset: {}",
                err
            )));
        }
    };

    // Extract and process columns
    let x_values = df
        .column(&data.x_column)
        .expect("X column not found")
        .utf8()
        .unwrap()
        .into_iter()
        .filter_map(|v| v.map(|s| s.to_string()))
        .collect::<Vec<_>>();

    let y_values = if let Some(y_col) = &data.y_column {
        df.column(y_col)
            .expect("Y column not found")
            .f64()
            .unwrap()
            .into_iter()
            .filter_map(|v| v)
            .collect::<Vec<_>>()
    } else {
        vec![] // For charts without Y-axis (e.g., Histogram)
    };

    // Optionally apply aggregation (if requested)
    let aggregated_values = if let Some(agg) = &data.aggregation {
        match agg.as_str() {
            "sum" => vec![y_values.iter().sum()],
            "avg" => vec![y_values.iter().sum::<f64>() / y_values.len() as f64],
            _ => y_values,
        }
    } else {
        y_values
    };

    // Package and send data
    let chart_data = ChartDataResponse {
        labels: x_values,
        values: aggregated_values,
    };

    Ok(HttpResponse::Ok().json(chart_data))
}

// Configure function for charts routes
pub fn charts_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/charts").route("/generate-chart", web::post().to(generate_chart)));
}
