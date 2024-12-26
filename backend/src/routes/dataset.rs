use crate::disk_layer::file_handler::write_dataset;
use crate::repositories::dataset_repository;
use crate::services::dataset_service::{convert_csv_to_parquet, generate_columns_metadata};
use crate::utils::jwt::get_user_id_from_request;
use crate::{models::datasets::Dataset, services::dataset_service::upload_to_supabase};
use actix_multipart::Multipart;
use actix_web::error::ErrorInternalServerError;
use actix_web::{
    error::InternalError,
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use chrono::Utc;
use futures_util::StreamExt;
use log::error;
use mime_guess;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use super::transformation::apply_transformation;

pub async fn upload_file_route(
    mut payload: Multipart,
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Iterate over the multipart fields
    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => {
                // Retrieve the file name from the headers
                let content_disposition = field.content_disposition();
                let file_name = content_disposition
                    .get_filename()
                    .unwrap_or("uploaded_file")
                    .to_string();

                if !is_file_format_allowed(&file_name) {
                    return Ok(HttpResponse::BadRequest().body("Unsupported file type"));
                }

                // Read the entire file into memory
                let mut file_data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk?;
                    file_data.extend_from_slice(&chunk);
                }

                if !is_file_size_allowed(&file_data) {
                    return Ok(HttpResponse::BadRequest().body("File size exceeds the limit"));
                }

                if file_data.is_empty() {
                    return Ok(HttpResponse::BadRequest().body("File is empty"));
                }

                let file_size = file_data.len() as i64; // File size in bytes
                let file_type = mime_guess::from_path(&file_name)
                    .first_or_octet_stream()
                    .to_string();

                let dataset_url = upload_to_supabase(file_name.clone(), file_data.clone()).await?;

                // let uploaded_by = get_user_id_from_request(&req);

                // Generate preview (entire dataset as JSON)
                let preview = generate_preview(&file_data, &file_name)?;

                // let column_metadata = generate_columns_metadata(&file_data, &file_name)?;

                let dataset = dataset_repository::insert_new_dataset(
                    &pool,
                    file_name,
                    file_size,
                    file_type,
                    dataset_url.clone(),
                    Some(Utc::now().naive_utc()),
                    None,
                    None,
                    preview,
                    Some(serde_json::Value::Null),
                )
                .await
                .map_err(|e| {
                    error!(
                        "Failed to insert dataset into the database: {}",
                        e.to_string()
                    );
                    InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                })?;

                // Store the dataset in the disk
                if let Some(dataset_id) = dataset.id {
                    let parquet_data = convert_csv_to_parquet(&file_data)?;
                    write_dataset(&format!("dataset-{}", dataset_id), &parquet_data).await?;
                }

                return Ok(HttpResponse::Ok().body(format!("File uploaded: {}", dataset_url)));
            }
            Err(e) => {
                return Ok(HttpResponse::InternalServerError()
                    .body(format!("Error processing field: {}", e)));
            }
        }
    }

    Ok(HttpResponse::BadRequest().body("No file uploaded"))
}

pub async fn get_datasets(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Query the datasets
    let datasets = dataset_repository::get_datasets(&pool).await.map_err(|e| {
        // Convert the error to an Actix-compatible error
        actix_web::error::ErrorInternalServerError(format!("Failed to retrieve datasets: {}", e))
    })?;

    // If datasets are empty, return an empty JSON array with type annotation
    if datasets.is_empty() {
        return Ok(HttpResponse::Ok().json(Vec::<Dataset>::new()));
    }

    Ok(HttpResponse::Ok().json(datasets))
}

pub async fn get_dataset_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let dataset_id = id.into_inner();
    // Query the dataset
    let dataset = dataset_repository::get_dataset_by_id(&pool, dataset_id)
        .await
        .map_err(|e| {
            // Convert the error to an Actix-compatible error
            actix_web::error::ErrorInternalServerError(format!("Failed to retrieve dataset: {}", e))
        })?;

    // Return the dataset as a JSON response
    Ok(HttpResponse::Ok().json(dataset))
}

// Configure function for file routes
pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/file").route("/upload", web::post().to(upload_file_route)));
    cfg.service(
        web::scope("/datasets")
            .route("", web::get().to(get_datasets))
            .route("/{id}", web::get().to(get_dataset_by_id))
            .route(
                "/{dataset_id}/apply-transformation",
                web::post().to(apply_transformation),
            ),
    );
}

fn is_file_format_allowed(file_name: &str) -> bool {
    let allowed_extensions = ["csv", "json", "xlsx"];
    allowed_extensions
        .iter()
        .any(|ext| file_name.ends_with(ext))
}

fn is_file_size_allowed(file_data: &Vec<u8>) -> bool {
    let max_file_size = 50 * 1024 * 1024; // 50MB
    if file_data.len() > max_file_size {
        return false;
    }
    return true;
}

fn generate_preview(file_data: &[u8], file_name: &str) -> Result<Option<Value>, actix_web::Error> {
    if file_name.ends_with(".csv") {
        // CSV Preview
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file_data);
        let mut preview = Vec::new();
        let headers = rdr
            .headers()
            .map_err(|e| {
                // Error when reading headers
                ErrorInternalServerError(format!("Failed to read CSV headers: {}", e))
            })?
            .clone(); // Clone headers to return them

        // Read the first 20 data rows
        for (i, result) in rdr.records().enumerate() {
            if i >= 20 {
                // Only take the first 20 rows
                break;
            }
            match result {
                Ok(record) => {
                    // Convert each &str element into String and push it to the preview
                    preview.push(
                        record
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    );
                }
                Err(err) => {
                    return Err(ErrorInternalServerError(format!(
                        "Failed to parse CSV record: {}",
                        err
                    )));
                }
            }
        }

        // Return preview with headers and data in JSON format
        return Ok(Some(json!({
            "headers": headers.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            "preview": preview
        })));
    } else if file_name.ends_with(".json") {
        // JSON Preview
        let json_data: Value = serde_json::from_slice(file_data).map_err(|e| {
            // Handle JSON parsing errors
            ErrorInternalServerError(format!("Failed to parse JSON: {}", e))
        })?;

        if let Some(array) = json_data.as_array() {
            let mut preview = Vec::new();
            let mut headers = Vec::new();

            // Extract headers from the first object in the array
            if let Some(first_object) = array.get(0) {
                headers = first_object
                    .as_object()
                    .unwrap_or(&serde_json::Map::new())
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>();
            }

            // Preview the first 20 rows
            for (i, item) in array.iter().take(20).enumerate() {
                if let Some(obj) = item.as_object() {
                    preview.push(obj.values().map(|v| v.to_string()).collect::<Vec<String>>());
                }
            }

            // Return preview with headers and data in JSON format
            return Ok(Some(json!({
                "headers": headers,
                "preview": preview
            })));
        }
    }

    // For unsupported formats (e.g., XLSX)
    Err(ErrorInternalServerError(
        "Unsupported file type for preview generation",
    ))
}
