use crate::disk_layer::file_handler::{read_dataset, write_dataset};
use crate::dto::datasets;
use crate::repositories::cell_repository::get_cell_by_id;
use crate::repositories::dataframe::get_dataframe_by_id;
use crate::repositories::dataset_repository::get_dataset_by_id;
use crate::services::cell_service::check_and_create_dataframe_id;
use crate::{
    models::cell::Cell,
    repositories::{
        cell_repository, dataframe::save_dataframe, transformations::save_transformation_history,
    },
    services::{
        dataframe_service::save_dataframe_to_supabase,
        transformation_service::{apply_transformations, load_dataset},
    },
};
use actix_web::{web, Error, HttpResponse};
use log::{error, info};
use polars::prelude::*;
use serde_json::Value;
use sqlx::PgPool;
use std::str::FromStr;
use tokio::fs;
use uuid::Uuid;

pub async fn apply_transformation(
    cell_id: web::Path<Uuid>,
    body: web::Json<Value>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let cell_id = cell_id.into_inner();
    let transformations = body.into_inner();

    let data_type_str = transformations["data_type"]
        .as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid data_type format"))?; // Ensure it's a valid string

    let input_data_type_str = transformations["input_data_type"]
        .as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid data_type format"))?; // Ensure it's a valid string

    let dataset_id_str = transformations["dataset_id"]
        .as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid dataset_id format"))?; // Ensure it's a valid string

    // Parse the string into a Uuid
    let dataset_id: Uuid = Uuid::parse_str(dataset_id_str).map_err(|e| {
        error!("Failed to parse dataset_id: {}", e);
        actix_web::error::ErrorBadRequest(format!("Failed to parse dataset_id: {}", e))
    })?;
    let mut dataset_metadata = None;
    let mut dataframe_metadata = None;
    if data_type_str == "dataset" {
        let ds = get_dataset_by_id(&pool, dataset_id).await.map_err(|e| {
            error!("Failed to get dataset: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to get dataset: {}", e))
        })?;
        dataset_metadata = Some(ds);
    } else if data_type_str == "dataframe" {
        let df = get_dataframe_by_id(&pool, dataset_id).await.map_err(|e| {
            error!("Failed to get dataframe: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to get dataframe: {}", e))
        })?;
        dataframe_metadata = Some(df);
    } else {
        return Err(actix_web::error::ErrorBadRequest("Invalid data type"));
    }
    // Step 1: Load dataset
    let dataset = load_dataset(dataset_id, data_type_str, &pool)
        .await
        .map_err(|e| {
            error!("Failed to load dataset: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to load dataset: {}", e))
        })?;

    // Step 2: Apply transformations
    let transformations_vec = vec![transformations["transformation"].clone()];
    let parquet_file_path = apply_transformations(
        dataset_id,
        dataset,
        transformations_vec,
        data_type_str,
        dataset_metadata.unwrap_or_default(),
        dataframe_metadata.unwrap_or_default(),
    )
    .map_err(|e| {
        error!("Failed to apply transformation: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to apply transformation: {}", e))
    })?;

    // Step 3: Save history
    let transformation = save_transformation_history(
        &pool,
        dataset_id,
        transformations["transformation"].clone(),
        data_type_str,
    )
    .await
    .map_err(|e| {
        error!("Failed to save transformation: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to save transformation: {}", e))
    })?;

    let dataframe_id = check_and_create_dataframe_id(&pool, cell_id)
        .await
        .map_err(|e| {
            error!("Failed creating dataframe id: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to save dataframe: {}", e))
        })?;

    let parquet_data = fs::read(&parquet_file_path).await?;
    info!(
        "Writing transformed dataframe, size before encryption: {} bytes",
        parquet_data.len()
    );
    // write_dataset(&format!("dataframe-{}", dataframe_id), &parquet_data).await?;
    // let _ = fs::remove_file(&parquet_file_path);

    // Explicitly handle the write with error checking
    match write_dataset(&format!("dataframe-{}", dataframe_id), &parquet_data).await {
        Ok(_) => {
            info!("Successfully wrote encrypted dataframe to disk");

            // Verify we can read it back immediately
            match read_dataset(&format!("dataframe-{}", dataframe_id)).await {
                Ok(verification_data) => {
                    info!(
                        "Verification read successful, size after decryption: {} bytes",
                        verification_data.len()
                    );
                }
                Err(e) => {
                    error!("Verification read failed: {}", e);
                    // Continue anyway as this is just verification
                }
            }
        }
        Err(e) => {
            error!("Failed to write encrypted dataframe: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Failed to write encrypted dataframe: {}",
                e
            )));
        }
    }

    // Remove temporary file
    if let Err(e) = fs::remove_file(&parquet_file_path).await {
        error!("Failed to remove temporary file: {}", e);
        // Continue anyway as this is cleanup
    }

    let dataframe = save_dataframe(&pool, dataframe_id, Some(transformation.id), String::new())
        .await
        .map_err(|e| {
            error!("Failed to save dataframe: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to save dataframe: {}", e))
        })?;

    let cell = match datasets::DataType::from_str(input_data_type_str) {
        Ok(datasets::DataType::Dataset) => Cell::new(
            cell_id,
            dataframe.transformation_id,
            None,
            Some(transformation.dataset_id),
            Some(dataframe.id),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Ok(datasets::DataType::DataFrame) => Cell::new(
            cell_id,
            dataframe.transformation_id,
            Some(transformation.dataset_id),
            None,
            Some(dataframe.id),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Err(err) => {
            info!("Error: {}", err);
            return Err(actix_web::error::ErrorBadRequest("Invalid data type"));
        }
    };

    let _ = cell_repository::insert_or_update_cell(&pool, cell).await;

    Ok(HttpResponse::Ok().json(dataframe_id))
}
