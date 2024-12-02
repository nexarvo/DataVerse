use crate::services::file_service::upload_to_supabase;
use actix_multipart::Multipart;
use actix_web::{
    web::{self, Data},
    Error, HttpResponse,
};
use futures_util::StreamExt;
use mime_guess;
use sqlx::PgPool;

pub async fn upload_file_route(
    mut payload: Multipart,
    pool: Data<PgPool>,
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

                let dataset_url = upload_to_supabase(file_name.clone(), file_data).await?;

                let _ = store_metadata(
                    pool.clone(),
                    file_name,
                    file_size,
                    file_type,
                    dataset_url.clone(),
                )
                .await;

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

// Configure function for file routes
pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/file").route("/upload", web::post().to(upload_file_route)));
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

async fn store_metadata(
    pool: web::Data<PgPool>,
    file_name: String,
    file_size: i64,
    file_type: String,
    dataset_url: String,
) -> Result<HttpResponse, Error> {
    let query = r#"
        INSERT INTO datasets (file_name, file_size, file_type, dataset_url)
        VALUES ($1, $2, $3, $4)
    "#;

    sqlx::query(query)
        .bind(file_name)
        .bind(file_size)
        .bind(file_type)
        .bind(dataset_url)
        .execute(pool.get_ref())
        .await
        .map_err(|err| {
            // Log the error for debugging purposes
            eprintln!("Failed to execute query: {:?}", err);
            actix_web::error::ErrorInternalServerError("Failed to store metadata")
        })?;

    Ok(HttpResponse::Ok().body("Metadata stored successfully"))
}
