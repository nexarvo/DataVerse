use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use super::datasets::DatasetMetadataDTO;

#[derive(Serialize)]
pub struct TransformationDTO {
    pub id: Option<Uuid>,
    pub dataset: DatasetMetadataDTO,
    pub transformation_type: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub applied_at: Option<NaiveDateTime>,
    pub applied_by: Option<uuid::Uuid>,
    pub parent_transformation_id: Option<Uuid>
}