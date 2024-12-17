use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DatasetMetadataDTO {
    pub id: Option<Uuid>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub file_type: Option<String>,
    pub upload_time: Option<NaiveDateTime>,
    pub uploaded_by: Option<Uuid>,
    pub dataset_url: Option<String>,
    pub row_count: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Dataset,
    DataFrame,
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "dataset" => Ok(DataType::Dataset),
            "dataframe" => Ok(DataType::DataFrame),
            _ => Err(format!("Unknown data type: {}", input)),
        }
    }
}
