use actix_web::{web, HttpResponse};
use log::{error, info};
use sqlx::PgPool;

use crate::{
    disk_layer::file_handler::write_dataset,
    dto::{datasets::DataType, sql_cell::SQLQueryRequest},
    repositories::{
        cell_repository, dataframe::get_dataframes_by_ids, dataset_repository::get_datasets_by_ids,
    },
    services::sql_cell_service::{
        load_inputs_datasets_dataframes_in_duckdb, run_query_with_duckdb,
        un_load_inputs_datasets_dataframes_in_duckdb, update_metadata,
    },
};

async fn sql_query_handler(
    pool: web::Data<PgPool>,
    req: web::Json<SQLQueryRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("Starting to process SQL query");
    let cell_id = &req.cell_id;
    let inputs = &req.inputs;
    let sql_query = &req.sql_query;

    // Get cell metadata from database
    let cell = cell_repository::get_cell_metadata_by_id(&pool, *cell_id)
        .await
        .map_err(|err| {
            error!("Failed to get cell: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to get cell")
        })?;

    if cell.is_none() {
        return Ok(HttpResponse::NotFound().json("Cell not found"));
    }

    // Get input dataset/dataframe from database
    let input_datasets = get_datasets_by_ids(
        &pool,
        inputs
            .iter()
            .filter(|input| input.data_type == DataType::Dataset.to_string())
            .map(|input| input.id)
            .collect(),
    )
    .await
    .map_err(|err| {
        error!("Failed to get input datasets: {}", err);
        actix_web::error::ErrorInternalServerError("Failed to get input datasets")
    })?;

    let input_dataframes = get_dataframes_by_ids(
        &pool,
        inputs
            .iter()
            .filter(|input| input.data_type == DataType::DataFrame.to_string())
            .map(|input| input.id)
            .collect(),
    )
    .await
    .map_err(|err| {
        error!("Failed to get input dataframes: {}", err);
        actix_web::error::ErrorInternalServerError("Failed to get input dataframes")
    })?;

    //Step 1: Load all the datasets/dataframes in duckdb
    load_inputs_datasets_dataframes_in_duckdb(
        cell_id.clone(),
        input_datasets.clone(),
        input_dataframes.clone(),
    )
    .await
    .map_err(|err| {
        error!(
            "Failed to load input datasets/dataframes in duckdb: {}",
            err
        );
        actix_web::error::ErrorInternalServerError(
            "Failed to load input datasets/dataframes in duckdb",
        )
    })?;

    //Step 2: Execute SQL query using DuckDB
    let result = run_query_with_duckdb(cell_id.clone(), sql_query).await?;

    //Step 3: Unload all the datasets/dataframes from duckdb
    un_load_inputs_datasets_dataframes_in_duckdb(cell_id.clone(), input_datasets, input_dataframes)
        .await
        .map_err(|err| {
            error!(
                "Failed to unload input datasets/dataframes from duckdb: {}",
                err
            );
            actix_web::error::ErrorInternalServerError(
                "Failed to unload input datasets/dataframes from duckdb",
            )
        })?;

    //Step 4: Update the metadata in database
    let dataframe_id = update_metadata(&pool, *cell_id, Some(cell.unwrap()), (*inputs).clone())
        .await
        .map_err(|err| {
            error!("Failed to update metadata: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to update metadata")
        })?;

    //Step 5: Store result in new/existing dataframe in disk
    write_dataset(&format!("dataframe-{}", dataframe_id), &result)
        .await
        .map_err(|err| {
            error!("Failed to write result to disk: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to write result to disk")
        })?;

    info!("Successfully processed SQL query");
    //Step 6: Return result dataframe_id
    Ok(HttpResponse::Ok().json(dataframe_id))
}

// Configure function for sql query routes
pub fn sql_query_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/sql").route("/run", web::post().to(sql_query_handler)));
}
