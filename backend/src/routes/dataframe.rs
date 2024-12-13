use crate::dto::dataframe::{DataFramePreview, GetDataFrameParams, PaginatedDataFrameResponse};
use crate::repositories::dataframe::get_dataframe_by_id;
use crate::services::dataframe_service::{download_parquet_from_supabase, read_parquet_to_dataframe};
use actix_web::{
    web::{self}, Error, HttpResponse
};
use log::{error, info};
use polars::prelude::*;
use polars::frame::DataFrame;
use sqlx::PgPool;


pub async fn get_dataframe(
    params: web::Query<GetDataFrameParams>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    info!("Starting to get dataframe for query: {}", params.dataframe_id);

    let dataframe_id = params.dataframe_id;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    // Step 1: Fetch the metadata from PostgreSQL (get the file URL)
    let dataframe = get_dataframe_by_id(&pool, dataframe_id).await.map_err(|e| {
        error!("Failed to fetch metadata: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to fetch metadata: {}", e))
    })?;

    match dataframe {
        Some(dataframe) => {
            // Proceed with the valid dataframe
            let file_path = download_parquet_from_supabase(dataframe_id, &dataframe.dataframe_duckdb_reference).await?;
            // Step 3: Read Parquet file into Polars DataFrame
            let df = read_parquet_to_dataframe(&file_path)?;

            // Step 4: Apply pagination to DataFrame
            let start_row = ((page - 1) * page_size as u32) as usize;

            // Step 5: Generate headers and preview (first few rows)
            let preview = generate_dataframe_preview(&df, start_row, page_size as usize, page);
            
            info!("Successfully get dataframe for query: {}", params.dataframe_id);
            // Step 6: Return the headers and preview as JSON with pagination metadata
            Ok(HttpResponse::Ok().json(preview))
        }
        None => {
            // Handle the case where no DataFrame was found (i.e., the query returned `None`)
            return Err(actix_web::error::ErrorNotFound("DataFrame not found"));
        }
    }
}

pub fn generate_dataframe_preview(
    df: &DataFrame, 
    start_row: usize, 
    page_size: usize, 
    page: u32,
) -> PaginatedDataFrameResponse {
    info!("Generating preview for dataframe");
    // Step 1: Get the headers (column names)
    let headers = df.get_columns()
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

    let latest_preview = DataFramePreview {headers, preview};

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
}
