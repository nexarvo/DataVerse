use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub inputs: Option<Value>,
    pub cell_type: Option<String>,
    pub cell_order: Option<i32>,
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
        cell_type: Option<String>,
        cell_order: Option<i32>,
        name: Option<String>,
        created_by: Option<Uuid>,
        updated_by: Option<Uuid>,
        inputs: Option<Value>,
    ) -> Self {
        let created_at = Some(chrono::Utc::now().naive_utc()); // Set creation time
        let updated_at = created_at; // Set updated time as the same as created time

        Self {
            id,
            name,
            first_transformation_id,
            input_dataframe_id,
            input_dataset_id,
            result_dataframe_id,
            created_at,
            created_by,
            updated_at,
            updated_by,
            cell_type,
            cell_order,
            inputs,
        }
    }

    // Builder-like method to update a Cell
    pub fn update(self) -> CellUpdater {
        CellUpdater::new(self)
    }

    // Optional: Method to set the name of the cell
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

pub struct CellUpdater {
    cell: Cell,
}

impl CellUpdater {
    pub fn new(cell: Cell) -> Self {
        Self { cell }
    }

    pub fn first_transformation_id(mut self, value: Uuid) -> Self {
        self.cell.first_transformation_id = Some(value);
        self
    }

    pub fn input_dataframe_id(mut self, value: Uuid) -> Self {
        self.cell.input_dataframe_id = Some(value);
        self
    }

    pub fn input_dataset_id(mut self, value: Uuid) -> Self {
        self.cell.input_dataset_id = Some(value);
        self
    }

    pub fn result_dataframe_id(mut self, value: Uuid) -> Self {
        self.cell.result_dataframe_id = Some(value);
        self
    }

    pub fn cell_type(mut self, value: String) -> Self {
        self.cell.cell_type = Some(value);
        self
    }

    pub fn cell_order(mut self, value: i32) -> Self {
        self.cell.cell_order = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.cell.name = Some(value);
        self
    }

    pub fn updated_by(mut self, value: Uuid) -> Self {
        self.cell.updated_by = Some(value);
        self
    }

    pub fn inputs(mut self, value: Value) -> Self {
        self.cell.inputs = Some(value);
        self
    }

    pub fn finish(mut self) -> Cell {
        self.cell.updated_at = Some(chrono::Utc::now().naive_utc());
        self.cell
    }
}
