use std::{fs, path::PathBuf};

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{
    dto::sql_cell::SQLQueryRequest,
    repositories::{dataframe::get_dataframe_by_id, dataset_repository::get_dataset_by_id},
    services::{
        dataframe_service::download_parquet_from_supabase, dataset_service::download_dataset,
        sql_cell_service::query_file_with_duckdb,
    },
};

async fn sql_query_handler(
    pool: web::Data<PgPool>,
    req: web::Json<SQLQueryRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let cell_id = &req.cell_id;
    let dataset_id = &req.dataset_id;
    let is_dataset = &req.is_dataset;
    let sql_query = &req.sql_query;

    // Step 1: Download the file
    let file_path = match is_dataset {
        true => {
            let dataset = get_dataset_by_id(&pool, *dataset_id).await.map_err(|e| {
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
            let dataframe = get_dataframe_by_id(&pool, *dataset_id).await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to retrieve dataframe: {}",
                    e
                ))
            })?;
            let dataframe_reference = match dataframe {
                Some(df) => df.dataframe_duckdb_reference,
                None => return Err(actix_web::error::ErrorNotFound("Dataframe not found")),
            };
            let result = download_parquet_from_supabase(*dataset_id, &dataframe_reference)
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

    // Step 2: Execute SQL query using DuckDB
    let dataframe_id = match query_file_with_duckdb(
        &pool,
        *cell_id,
        *dataset_id,
        *is_dataset,
        &file_path,
        sql_query,
    )
    .await
    {
        Ok(results) => results,
        Err(err) => {
            // Cleanup temp file before returning error
            let _ = fs::remove_file(&file_path);
            return Ok(HttpResponse::InternalServerError()
                .json(format!("Query execution failed: {}", err)));
        }
    };

    // Step 3: Cleanup temp file
    let _ = fs::remove_file(&file_path);

    // Step 4: Return results
    Ok(HttpResponse::Ok().json(dataframe_id))
}

// Configure function for sql query routes
pub fn sql_query_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/sql").route("/run", web::post().to(sql_query_handler)));
}
