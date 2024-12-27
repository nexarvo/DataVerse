use actix_web::web::Bytes;
use log::{error, info, warn};
use sqlx::PgPool;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::db::duck_db_connection::get_duckdb_connection;
use crate::dto::cell::CellSQLInputsModal;
use crate::models::cell::Cell;
use crate::models::dataframe::DataFrame as DataFrameModel;
use crate::models::datasets::Dataset;
use crate::repositories::cell_repository;
use crate::repositories::dataframe::save_dataframe;
use crate::services::cell_service::check_and_create_dataframe_id;

/// Executes a SQL query on a DuckDB database, exports the result as a Parquet file to a temporary location,
/// and returns the result as a byte array.
///
/// # Arguments
///
/// * `cell_id` - A unique identifier (`Uuid`) for the cell for which the query is being executed.
/// * `sql_query` - The SQL query string to be executed on the DuckDB database.
///
/// # Returns
///
/// This function returns a `Result`:
/// - `Ok(Bytes)` - A `Bytes` object containing the Parquet data resulting from the executed query.
/// - `Err(Box<dyn std::error::Error>)` - An error is returned if any step of the process fails, including spawning the DuckDB process, reading the temporary Parquet file, or closing the file.
///
/// # Errors
///
/// This function may return various errors, such as:
/// - Failure to spawn the DuckDB process.
/// - Failure to read the Parquet file from the temporary file.
/// - Any other I/O or DuckDB related errors that may occur during execution.
///
/// # Example
///
/// ```rust
/// let result = run_query_with_duckdb(cell_id, "SELECT * FROM my_table WHERE value > 10").await;
/// match result {
///     Ok(parquet_data) => {
///         // Process the Parquet data
///     }
///     Err(e) => {
///         // Handle the error
///     }
/// }
/// ```
///
/// # Detailed Steps:
/// 1. A temporary file is created to store the result of the DuckDB query in Parquet format.
/// 2. The DuckDB process is spawned with the specified query, exporting the result to the temporary file in Parquet format.
/// 3. If the DuckDB process is successful, the Parquet file is read into memory.
/// 4. The temporary file is closed and the Parquet data is returned as a `Bytes` object.
pub async fn run_query_with_duckdb(
    cell_id: Uuid,
    sql_query: &str,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    info!("Querying file with DuckDB for cell_id: {}", cell_id);

    info!(
        "Executing SQL query for cell_id: {}, query: {}",
        cell_id, sql_query
    );

    // Create a temporary file
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_string_lossy().to_string();

    // Build the DuckDB command to export query results to the temporary file
    let command_status = Command::new("duckdb")
        .arg("dataverse_duckdb.db")
        .arg("-c")
        .arg(format!(
            "COPY ({}) TO '{}' (FORMAT PARQUET)",
            sql_query, temp_path
        ))
        .status()
        .map_err(|e| {
            error!("Failed to spawn DuckDB process: {:?}", e);
            e
        })?;

    // Check if the command was successful
    if !command_status.success() {
        return Err("DuckDB process failed".into());
    }

    // Read the Parquet file from the temporary file
    let parquet_data = fs::read(&temp_path).map_err(|e| {
        error!("Failed to read Parquet file from temp file: {:?}", e);
        e
    })?;

    info!(
        "Successfully queried file with DuckDB for cell_id: {} and returned as Parquet binary",
        cell_id
    );

    temp_file.close()?;

    // Return the data as bytes
    Ok(Bytes::from(parquet_data))
}

pub async fn load_inputs_datasets_dataframes_in_duckdb(
    cell_id: Uuid,
    input_datasets: Vec<Dataset>,
    input_dataframes: Vec<DataFrameModel>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Loading all the datasets/dataframes in DuckDB for cell_id: {}",
        cell_id
    );

    // Helper closure to load datasets or dataframes
    let load_into_duckdb = |id: Uuid,
                            file_prefix: &str,
                            table_prefix: &str|
     -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("./data/{}-{:.0}.parquet", file_prefix, id);

        let conn = get_duckdb_connection()?;

        let table_name: String = format!("\"{}\"", table_prefix.to_owned());
        info!(
            "Creating table for {}: {:?}, table_name: {} and getting data from file: {}",
            file_prefix, id, table_name, file_path
        );

        // Attach the file as a table
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM read_parquet('{}')",
                table_name, file_path
            ),
            [],
        )?;
        info!(
            "Successfully created table for {}_id: {:?}",
            file_prefix, id
        );

        Ok(())
    };

    // Load datasets
    for dataset in input_datasets {
        if let Some(id) = dataset.id {
            load_into_duckdb(id, "dataset", &dataset.file_name)?;
        }
    }

    // Load dataframes
    for dataframe in input_dataframes {
        if let Some(name) = &dataframe.name {
            load_into_duckdb(dataframe.id, "dataframe", name)?;
        }
    }

    info!(
        "Successfully loaded all the datasets/dataframes in DuckDB for cell_id: {}",
        cell_id
    );
    Ok(())
}

pub async fn un_load_inputs_datasets_dataframes_in_duckdb(
    cell_id: Uuid,
    input_datasets: Vec<Dataset>,
    input_dataframes: Vec<DataFrameModel>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Unloading all the datasets/dataframes in DuckDB for cell_id: {}",
        cell_id
    );

    let conn = get_duckdb_connection()?;

    // Helper closure to unload datasets or dataframes
    let unload_from_duckdb = |table_prefix: &str| -> Result<(), Box<dyn std::error::Error>> {
        let table_name: String = format!("\"{}\"", table_prefix);
        info!("Dropping table: {}", table_name);

        // Drop the table
        conn.execute(&format!("DROP TABLE IF EXISTS {}", table_name), [])?;
        info!("Successfully dropped table: {}", table_name);

        Ok(())
    };

    // Unload datasets
    for dataset in input_datasets {
        unload_from_duckdb(&dataset.file_name)?;
    }

    // Unload dataframes
    for dataframe in input_dataframes {
        if let Some(name) = &dataframe.name {
            unload_from_duckdb(name)?;
        }
    }

    info!(
        "Successfully unloaded all the datasets/dataframes in DuckDB for cell_id: {}",
        cell_id
    );
    Ok(())
}

pub async fn update_metadata(
    pool: &PgPool,
    cell_id: Uuid,
    cell: Option<Cell>,
    inputs: Vec<CellSQLInputsModal>,
) -> Result<String, Box<dyn std::error::Error>> {
    info!("Updating metadata for cell_id: {}", cell_id);

    //TODO: update the transformation table schema for SQL transformations

    let dataframe_id = check_and_create_dataframe_id(&pool, cell_id)
        .await
        .map_err(|e| {
            error!("Failed creating dataframe id: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to save dataframe: {}", e))
        })?;

    let dataframe = save_dataframe(&pool, dataframe_id, None, "".to_owned())
        .await
        .map_err(|e| {
            error!("Failed to save dataframe metadata: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to save dataframe metadata: {}",
                e
            ))
        })?;

    if let Some(actual_cell) = cell {
        let updated_cell = actual_cell
            .update()
            .inputs(serde_json::to_value(inputs)?)
            .result_dataframe_id(dataframe_id)
            .finish();

        // Save the updated cell
        let _ = cell_repository::insert_or_update_cell(&pool, updated_cell).await;
    } else {
        // Handle the None case here
        warn!("Cell is None, cannot update.");
    }

    info!("Successfully updated metadata for cell_id: {}", cell_id);
    Ok(dataframe_id.to_string())
}
