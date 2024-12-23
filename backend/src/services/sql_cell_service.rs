use actix_web::HttpResponse;
use duckdb::types::ValueRef;
use log::{error, info};
use polars::error::PolarsError;
use polars::frame::DataFrame;
use polars::prelude::NamedFrom;
use polars::series::Series;
use serde_json::json;
use sqlx::{Execute, PgPool};
use std::path::PathBuf;
use uuid::Uuid;

use crate::db::duck_db_connection::DUCKDB_CONN;
use crate::repositories::dataframe::save_dataframe;
use crate::repositories::transformations::save_transformation_history;
use crate::services::cell_service::check_and_create_dataframe_id;

use super::dataframe_service::save_dataframe_to_supabase;
use super::transformation_service::save_as_parquet;

pub async fn query_file_with_duckdb(
    pool: &PgPool,
    cell_id: Uuid,
    dataset_id: Uuid,
    is_dataset: bool,
    file_path: &PathBuf,
    sql_query: &str,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    info!("Querying file with DuckDB for dataset_id: {}", dataset_id);
    let conn = DUCKDB_CONN
        .lock()
        .expect("Failed to acquire connection lock");

    let table_name = format!("\"dataset_{}\"", dataset_id);
    info!(
        "Creating table for dataset_id: {}, table_name: {}",
        dataset_id, table_name
    );
    // Attach the file as a table
    if is_dataset {
        // If it's a CSV file, use read_csv_auto
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM read_csv_auto('{}')",
                table_name,
                file_path.display()
            ),
            [],
        )?;
    } else {
        // If it's a Parquet file, use read_parquet
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM read_parquet('{}')",
                table_name,
                file_path.display()
            ),
            [],
        )?;
    }
    info!("Successfully created table for dataset_id: {}", dataset_id);

    match conn.execute(sql_query, []) {
        Ok(_) => info!("Query executed successfully without preparation"),
        Err(e) => error!("Query execution error: {:?}", e),
    }

    /////////////////////////////
    ///
    ///
    info!(
        "Executing SQL query for dataset_id: {}, query: {}",
        dataset_id, sql_query
    );

    // Prepare the query and immediately execute it
    let mut stmt = conn.prepare(sql_query).map_err(|e| {
        error!("Failed to prepare query: {:?}", e);
        e
    })?;

    // Retrieve column metadata before rows iteration
    let column_names = stmt.column_names().to_vec();
    if column_names.is_empty() {
        return Err("Failed to retrieve column names".into());
    }

    let column_count = column_names.len();
    info!(
        "Successfully executed SQL query for dataset_id: {}, columns: {:?}",
        dataset_id, column_names
    );

    let mut rows = stmt.query([]).map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        e
    })?;

    // Collect records into a vector
    let mut records: Vec<Vec<String>> = Vec::new();
    while let Some(row) = rows.next()? {
        let record = (0..column_count)
            .map(|i| match row.get_ref_unwrap(i) {
                ValueRef::Null => "".to_string(),
                ValueRef::Int(v) => v.to_string(),
                ValueRef::BigInt(v) => v.to_string(),
                ValueRef::Float(v) => v.to_string(),
                ValueRef::Double(v) => v.to_string(),
                ValueRef::Text(v) => String::from_utf8_lossy(v).to_string(),
                ValueRef::Date32(v) => v.to_string(),
                ValueRef::Time64(v, unit) => format!("{:?} {:?}", v, unit),
                ValueRef::Timestamp(v, unit) => format!("{:?} {:?}", v, unit),
                ValueRef::Boolean(v) => v.to_string(),
                _ => "".to_string(),
            })
            .collect();
        records.push(record);
    }

    // Convert collected records into Polars DataFrame
    info!(
        "Converting to Polars DataFrame for dataset_id: {}",
        dataset_id
    );

    let df = DataFrame::new(
        column_names
            .into_iter()
            .enumerate()
            .map(|(col_idx, name)| {
                let column_data: Vec<String> = records
                    .iter()
                    .map(|record| record[col_idx].clone())
                    .collect();

                // Explicit type annotation for Result<Series, PolarsError>
                Ok::<Series, PolarsError>(Series::new(&name, column_data))
            })
            .collect::<Result<Vec<_>, _>>()?, // The error type is now inferred to be PolarsError
    )?;

    info!(
        "Successfully converted to Polars DataFrame for dataset_id: {}",
        dataset_id
    );

    // Save DataFrame as a Parquet file and handle errors
    let file_path = format!("/tmp/{}.parquet", dataset_id);
    match save_as_parquet(df, &file_path) {
        Ok(()) => {}
        Err(e) => return Err(Box::new(e)), // Convert PolarsError to Box<dyn std::error::Error>
    }

    // Step 3: Save transformation history
    let transformation = save_transformation_history(
        &pool,
        dataset_id,
        json!({"type": "sql", "params": sql_query}),
        "dataframe", //We don't want to store dataset_id in transformation
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

    let (dataframe_id, file_url) = save_dataframe_to_supabase(dataframe_id, file_path)
        .await
        .map_err(|e| {
            error!("Failed to save dataframe to supabase: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to save dataframe to supabase: {}",
                e
            ))
        })?;

    let dataframe = save_dataframe(&pool, dataframe_id, transformation.id, file_url)
        .await
        .map_err(|e| {
            error!("Failed to save dataframe metadata: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to save dataframe metadata: {}",
                e
            ))
        })?;

    // let cell = match datasets::DataType::from_str(input_data_type_str) {
    //     Ok(datasets::DataType::Dataset) => Cell::new(
    //         cell_id,
    //         Some(dataframe.transformation_id),
    //         None,
    //         Some(transformation.dataset_id),
    //         Some(dataframe.id),
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //     ),
    //     Ok(datasets::DataType::DataFrame) => Cell::new(
    //         cell_id,
    //         Some(dataframe.transformation_id),
    //         Some(transformation.dataset_id),
    //         None,
    //         Some(dataframe.id),
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //     ),
    //     Err(err) => {
    //         info!("Error: {}", err);
    //         return Err(actix_web::error::ErrorBadRequest("Invalid data type"));
    //     }
    // };

    // let _ = cell_repository::insert_or_update_cell(&pool, cell).await;

    // Drop the DuckDB table
    conn.execute(&format!("DROP TABLE {}", table_name), [])?;

    info!(
        "Successfully queried file with DuckDB for dataset_id: {}",
        dataset_id
    );
    Ok(dataframe_id)
}
