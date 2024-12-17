use serde::Serialize;
use sqlx::{FromRow, Type};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

#[derive(Serialize, Debug, FromRow)]
pub struct DataFrame {
    pub id: Uuid,           
    pub name: Option<String>,                         // UUID for the dataframe (primary key)
    pub transformation_id: Uuid,                      // Foreign key to the transformations table
    pub dataframe_duckdb_reference: String,             // Reference to the DuckDB data frame
    pub created_at: Option<NaiveDateTime>,                    // Timestamp when the dataframe was created
    pub created_by: Option<Uuid>,                     // Optional reference to the user who created the dataframe
    pub updated_at: Option<NaiveDateTime>,                    // Timestamp when the dataframe was last updated
    pub updated_by: Option<Uuid>,                     // Optional reference to the user who last updated the dataframe
}

impl DataFrame {
    pub fn new(
        id: Uuid,
        name: Option<String>,
        transformation_id: Uuid,
        dataframe_duckdb_reference: String
    ) -> Self {
        let created_by = None;
        let updated_by = None;
        DataFrame {
            id,
            name,
            transformation_id,
            dataframe_duckdb_reference,
            created_at: Some(Utc::now().naive_utc()),
            created_by,
            updated_at: Some(Utc::now().naive_utc()),
            updated_by,
        }
    }
}