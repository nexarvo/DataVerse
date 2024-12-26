use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dataset {
    pub id: Option<Uuid>,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub upload_time: Option<NaiveDateTime>,
    pub uploaded_by: Option<Uuid>,
    pub dataset_url: String,
    pub row_count: Option<i32>,
    pub column_metadata: Option<Value>,
    pub latest_preview: Option<Value>,
}
