use duckdb::Connection;

pub fn get_duckdb_connection() -> Result<Connection, Box<dyn std::error::Error>> {
    // Open a disk-backed database file
    let conn = Connection::open("dataverse_duckdb.db")?;
    Ok(conn)
}
