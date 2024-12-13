use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Transformation {
    pub id: uuid::Uuid,
    pub dataset_id: uuid::Uuid,
    pub transformation_type: String,
    pub parameters: serde_json::Value,
    pub applied_at: chrono::DateTime<Utc>,
    pub applied_by: Option<uuid::Uuid>,
    pub result_preview: Option<serde_json::Value>,
    pub parent_transformation_id: Option<Uuid>
}
