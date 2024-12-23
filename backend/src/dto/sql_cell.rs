use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SQLQueryRequest {
    pub cell_id: Uuid,
    pub dataset_id: Uuid,
    pub is_dataset: bool,
    pub sql_query: String,
}

#[derive(Serialize)]
pub struct SQLQueryResponse {
    pub results: Vec<serde_json::Value>,
}
