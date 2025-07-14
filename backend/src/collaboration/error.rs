use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollaborationError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] axum::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Notebook not found")]
    NotebookNotFound,
    
    #[error("Operation conflict")]
    OperationConflict,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for CollaborationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CollaborationError::WebSocket(e) => {
                (StatusCode::BAD_REQUEST, format!("WebSocket error: {}", e))
            }
            CollaborationError::Serialization(e) => {
                (StatusCode::BAD_REQUEST, format!("Serialization error: {}", e))
            }
            CollaborationError::SessionNotFound => {
                (StatusCode::NOT_FOUND, "Session not found".to_string())
            }
            CollaborationError::NotebookNotFound => {
                (StatusCode::NOT_FOUND, "Notebook not found".to_string())
            }
            CollaborationError::OperationConflict => {
                (StatusCode::CONFLICT, "Operation conflict".to_string())
            }
            CollaborationError::PermissionDenied => {
                (StatusCode::FORBIDDEN, "Permission denied".to_string())
            }
            CollaborationError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {}", msg))
            }
        };

        let body = serde_json::to_string(&ErrorResponse {
            error: error_message.clone(),
            details: None,
        })
        .unwrap_or_else(|_| format!("\"error\": \"{}\"", error_message));

        (status, body).into_response()
    }
}

impl From<std::io::Error> for CollaborationError {
    fn from(err: std::io::Error) -> Self {
        CollaborationError::Internal(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for CollaborationError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        CollaborationError::Internal(err.to_string())
    }
}

impl From<tokio::sync::mpsc::error::SendError<axum::extract::ws::Message>> for CollaborationError {
    fn from(err: tokio::sync::mpsc::error::SendError<axum::extract::ws::Message>) -> Self {
        CollaborationError::Internal(format!("Failed to send message: {}", err))
    }
}
