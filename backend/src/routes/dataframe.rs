use std::io::Cursor;

use crate::disk_layer::file_handler::read_dataset;
use crate::dto::dataframe::{DataFramePreview, GetDataFrameParams, PaginatedDataFrameResponse};
use crate::repositories::dataframe::{get_dataframe_by_id, get_dataframes};
use crate::services::dataframe_service::{
    download_parquet_from_supabase, read_parquet_to_dataframe,
};
use actix_web::HttpRequest;
use actix_web::{
    web::{self},
    Error, HttpResponse,
};
use log::{error, info};
use polars::frame::DataFrame;
use polars::prelude::*;
use sqlx::PgPool;

pub async fn get_dataframe(
    params: web::Query<GetDataFrameParams>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    info!(
        "Starting to get dataframe for query: {}",
        params.dataframe_id
    );

    let dataframe_id = params.dataframe_id;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    // Step 1: Fetch the metadata from PostgreSQL
    let dataframe = get_dataframe_by_id(&pool, dataframe_id)
        .await
        .map_err(|e| {
            error!("Failed to fetch metadata: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to fetch metadata: {}", e))
        })?;

    // Handle the case where no DataFrame was found
    let dataframe = dataframe.ok_or_else(|| {
        error!("DataFrame not found for ID: {}", dataframe_id);
        actix_web::error::ErrorNotFound("DataFrame not found")
    })?;

    // Step 2: Attempt to read the dataset from disk
    let dataframe_key = format!("dataframe-{}", dataframe_id);
    let df = match read_dataset(&dataframe_key).await {
        Ok(parquet_data) => {
            // Read Parquet data from disk
            ParquetReader::new(Cursor::new(parquet_data))
                .finish()
                .map_err(|e| {
                    error!("Failed to read Parquet data: {}", e);
                    actix_web::error::ErrorInternalServerError(format!(
                        "Failed to read Parquet data: {}",
                        e
                    ))
                })?
        }
        Err(_) => {
            // Data not available on disk, download from Supabase
            info!("Data not found on disk. Attempting to download from Supabase.");
            let file_path =
                download_parquet_from_supabase(dataframe_id, &dataframe.dataframe_duckdb_reference)
                    .await
                    .map_err(|e| {
                        error!("Failed to download Parquet file: {}", e);
                        actix_web::error::ErrorInternalServerError(format!(
                            "Failed to download Parquet file: {}",
                            e
                        ))
                    })?;

            // Read the downloaded Parquet file
            read_parquet_to_dataframe(&file_path).map_err(|e| {
                error!("Failed to read downloaded Parquet file: {}", e);
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to read downloaded Parquet file: {}",
                    e
                ))
            })?
        }
    };

    // Step 3: Apply pagination
    let start_row = ((page - 1) * page_size as u32) as usize;

    // Step 4: Generate headers and preview
    let preview = generate_dataframe_preview(&df, start_row, page_size as usize, page);

    info!(
        "Successfully retrieved dataframe for query: {}",
        params.dataframe_id
    );

    // Step 5: Return the headers and preview as JSON
    Ok(HttpResponse::Ok().json(preview))
}

pub async fn get_dataframe_metadata(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Query the dataset
    let dataframes = get_dataframes(&pool).await.map_err(|e| {
        // Convert the error to an Actix-compatible error
        actix_web::error::ErrorInternalServerError(format!("Failed to retrieve dataframes: {}", e))
    })?;

    // Return the dataset as a JSON response
    Ok(HttpResponse::Ok().json(dataframes))
}

pub fn generate_dataframe_preview(
    df: &DataFrame,
    start_row: usize,
    page_size: usize,
    page: u32,
) -> PaginatedDataFrameResponse {
    info!("Generating preview for dataframe");
    // Step 1: Get the headers (column names)
    let headers = df
        .get_columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect::<Vec<String>>();

    // Step 2: Get the paginated rows (preview)
    let end_row = start_row + page_size;
    let paginated_df = df.slice(start_row as i64, page_size);

    // Step 3: Convert the DataFrame rows to a vector of string values
    let mut preview = Vec::new();

    // Iterate over each row in the DataFrame
    for row_idx in 0..paginated_df.height() {
        let mut row = Vec::new();

        // For each column in the DataFrame, convert the value to a string
        for col in paginated_df.get_columns() {
            let value = &col.get(row_idx);
            row.push(value.to_string());
        }

        preview.push(row);
    }

    // Get the total rows in the DataFrame (for pagination metadata)
    let total_rows = df.height() as u32;

    let latest_preview = DataFramePreview { headers, preview };

    info!("Successfully generated preview for dataframe");
    // Return the headers and preview along with pagination metadata
    PaginatedDataFrameResponse {
        latest_preview,
        total_rows,
        page,
        page_size: page_size as u32,
    }
}

// Configure function for dataframe routes
pub fn dataframe_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/dataframe").route("", web::get().to(get_dataframe)));
    cfg.service(
        web::scope("/dataframes-metadata").route("", web::get().to(get_dataframe_metadata)),
    );
}
