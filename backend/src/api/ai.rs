use crate::ai::copilot::{AICopilot, CodeAnalysis, CodeSuggestion};
use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CodeCompletionRequest {
    pub code: String,
    pub language: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CodeAnalysisRequest {
    pub code: String,
    pub language: String,
}

pub async fn get_code_completion(
    State(copilot): State<Arc<AICopilot>>,
    Json(payload): Json<CodeCompletionRequest>,
) -> impl IntoResponse {
    match copilot
        .get_code_completion(&payload.code, &payload.language, payload.context.as_deref())
        .await
    {
        Ok(suggestion) => (StatusCode::OK, Json(ApiResponse::success(suggestion))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to get code completion: {}", e))),
        ),
    }
}

pub async fn analyze_code(
    State(copilot): State<Arc<AICopilot>>,
    Json(payload): Json<CodeAnalysisRequest>,
) -> impl IntoResponse {
    match copilot.analyze_code(&payload.code, &payload.language).await {
        Ok(analysis) => (StatusCode::OK, Json(ApiResponse::success(analysis))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to analyze code: {}", e))),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<AICopilot>> {
    use axum::routing::*;

    Router::new()
        .route("/completion", post(get_code_completion))
        .route("/analyze", post(analyze_code))
}
