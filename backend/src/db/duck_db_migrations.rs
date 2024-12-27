use log::info;

use crate::db::duck_db_connection::get_duckdb_connection;

pub fn run_duckdb_migrations() {
    info!("Starting to run duckdb migrations");
    let conn = get_duckdb_connection().expect("Failed to acquire DuckDB connection");

    // Create the datasets table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS datasets (
            dataset_id TEXT PRIMARY KEY,
            dataset_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )
    .expect("Failed to run DuckDB migrations");

    // Create the data_frames table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data_frames (
            data_frame_id TEXT PRIMARY KEY,
            file_path TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )
    .expect("Failed to run DuckDB migrations");

    info!("Successfully ran duckdb migrations");
}
