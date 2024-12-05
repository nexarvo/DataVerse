use crate::services::transformation_service::{
    apply_transformations, load_dataset, save_transformation_history,
};
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
    let mut transformed_data =
        apply_transformations(dataset, transformations_vec).map_err(|e| {
            error!("Failed to apply transformation: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to apply transformation: {}",
                e
            ))
        })?;

    // Step 3: Save history
    save_transformation_history(&pool, dataset_id, &transformations, &transformed_data)
        .await
        .map_err(|e| {
            error!("Failed to save history: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to save history: {}", e))
        })?;

    // Step 4: Serialize DataFrame to JSON
    let mut json_data = Vec::new();
    {
        let writer = JsonWriter::new(&mut json_data);
        writer
            .with_json_format(JsonFormat::Json)
            .finish(&mut transformed_data)
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to serialize DataFrame: {}",
                    e
                ))
            })?;
    }

    // Step 5: Return transformed data as JSON
    let json_string = String::from_utf8(json_data).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to convert JSON data: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(json_string))
}

// Register all transformation routes
pub fn transformation_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/datasets/{dataset_id}/apply-transformation",
        web::post().to(apply_transformation),
    );
}
