use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::transformation::TransformationDTO;

#[derive(Deserialize)]
pub struct GetDataFrameParams {
    pub dataframe_id: Uuid,
    pub page: Option<u32>,    // Page number (default to 1)
    pub page_size: Option<u32>, // Page size (default to 20)
}

#[derive(Serialize)]
pub struct PaginatedDataFrameResponse {
    pub latest_preview: DataFramePreview,
    pub total_rows: u32,         
    pub page: u32,               
    pub page_size: u32,          
}

#[derive(Serialize)]
pub struct DataFramePreview {
    pub headers: Vec<String>,  // List of column names (headers)
    pub preview: Vec<Vec<String>>, // Preview rows (as vectors of strings)
}

#[derive(Serialize)]
pub struct DataframeMetadataDTO {
    pub id: Uuid,
    pub name: Option<String>,
    pub transformations: Vec<TransformationDTO>,
    pub dataframe_duckdb_reference: String,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<Uuid>,    
}

