use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cell {
    pub id: Uuid,
    pub name: Option<String>,
    pub first_transformation_id: Option<Uuid>,
    pub input_dataframe_id: Option<Uuid>,
    pub input_dataset_id: Option<Uuid>,
    pub result_dataframe_id: Option<Uuid>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<Uuid>,
}

impl Cell {
    // Constructor function to create a new Cell instance
    pub fn new(
        id: Uuid,
        first_transformation_id: Option<Uuid>,
        input_dataframe_id: Option<Uuid>,
        input_dataset_id: Option<Uuid>,
        result_dataframe_id: Option<Uuid>,
        created_by: Option<Uuid>,
        updated_by: Option<Uuid>,
    ) -> Self {
        let created_at = Some(chrono::Utc::now().naive_utc()); // Set creation time
        let updated_at = created_at; // Set updated time as the same as created time

        Self {
            id,
            name: None, // Name can be set later
            first_transformation_id,
            input_dataframe_id,
            input_dataset_id,
            result_dataframe_id,
            created_at,
            created_by,
            updated_at,
            updated_by,
        }
    }

    // Optional: Method to set the name of the cell
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}
