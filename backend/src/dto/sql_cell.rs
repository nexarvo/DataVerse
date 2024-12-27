use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::cell::CellSQLInputsModal;

#[derive(Deserialize, Clone)]
pub struct SQLQueryRequest {
    pub cell_id: Uuid,
    pub inputs: Vec<CellSQLInputsModal>,
    pub sql_query: String,
}

#[derive(Serialize)]
pub struct SQLQueryResponse {
    pub results: Vec<serde_json::Value>,
}
