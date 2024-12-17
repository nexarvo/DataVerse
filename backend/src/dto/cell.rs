use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use super::{dataframe::DataframeMetadataDTO, datasets::DatasetMetadataDTO, transformation::TransformationDTO};

#[derive(Serialize)]
pub struct CellDTO {
    pub id: Uuid,
    pub name: Option<String>,
    pub input_dataframe: Option<DataframeMetadataDTO>,
    pub input_dataset: Option<DatasetMetadataDTO>,
    pub result_dataframe: Option<DataframeMetadataDTO>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<Uuid>,
}