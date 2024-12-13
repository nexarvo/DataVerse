use duckdb::Connection;
use std::sync::Mutex;
use lazy_static::lazy_static;

// This is the global DuckDB connection available across the project
lazy_static! {
    pub static ref DUCKDB_CONN: Mutex<Connection> = Mutex::new(
        Connection::open("dataverse_duckdb.db").expect("Failed to connect to DuckDB")
    );
}