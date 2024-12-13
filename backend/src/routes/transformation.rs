use crate::{repositories::dataframe::save_dataframe, services::{dataframe_service::save_dataframe_to_supabase, transformation_service::{
    apply_transformations, load_dataset
}}, repositories::transformations::save_transformation_history};
use actix_web::{web, Error, HttpResponse};
use log::error;
use polars::prelude::*;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn apply_transformation(
    dataset_id: web::Path<Uuid>,
    body: web::Json<Value>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let dataset_id = dataset_id.into_inner();
    let transformations = body.into_inner();

    // Step 1: Load dataset
    let dataset = load_dataset(dataset_id, &pool).await.map_err(|e| {
        error!("Failed to load dataset: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to load dataset: {}", e))
    })?;

    // Step 2: Apply transformations
    let transformations_vec = vec![transformations.clone()];
    let parquet_file_path =
        apply_transformations(dataset_id, dataset, transformations_vec).map_err(|e| {
            error!("Failed to apply transformation: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to apply transformation: {}",
                e
            ))
        })?;

    // Step 3: Save history
    let transformation = save_transformation_history(&pool, dataset_id, &transformations)
        .await
        .map_err(|e| {
            error!("Failed to save transformation: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to save transformation: {}", e))
        })?;

    let (dataframe_id, file_url) = save_dataframe_to_supabase(parquet_file_path)
        .await
        .map_err(|e| {
            error!("Failed to save dataframe: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to save dataframe: {}", e))
        })?;

    let _ = save_dataframe(&pool, dataframe_id, transformation.id, file_url).await;

    Ok(HttpResponse::Ok().json(dataframe_id))
}
