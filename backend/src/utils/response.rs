use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub status_code: u16,
    pub message: String,
    pub token: Option<String>,
}
