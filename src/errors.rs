use axum::response::IntoResponse;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Process failed: {0}")]
    ProcessFailed(String),
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
}

impl IntoResponse for DomainError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            DomainError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, msg.to_string()),
            DomainError::InvalidInput(msg) => {
                (axum::http::StatusCode::BAD_REQUEST, msg.to_string())
            }
            DomainError::ProcessFailed(msg) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.to_string(),
            ),
            DomainError::InferenceFailed(msg) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.to_string(),
            ),
        };
        let body = axum::Json(serde_json::json!({"error": message}));
        (status, body).into_response()
    }
}
