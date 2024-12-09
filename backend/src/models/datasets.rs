use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Dataset {
    pub id: Option<Uuid>,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub upload_time: Option<NaiveDateTime>,
    pub uploaded_by: Option<Uuid>,
    pub dataset_url: String,
    pub row_count: Option<i32>,
}
