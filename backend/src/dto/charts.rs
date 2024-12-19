use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ChartRequest {
    pub dataset_id: Uuid,            // Dataset identifier
    pub is_dataset: bool,            // Is dataset or dataframe
    pub chart_type: String,          // Bar Chart, Line Chart, etc.
    pub x_column: String,            // Column for X-axis
    pub y_column: Option<String>,    // Optional column for Y-axis
    pub aggregation: Option<String>, // Optional aggregation (e.g., sum, avg)
}

#[derive(Serialize)]
pub struct ChartDataResponse {
    pub labels: Vec<String>, // X-axis labels
    pub values: Vec<f64>,    // Y-axis values
}
