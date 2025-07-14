use crate::data::profiler::{DataProfiler, DataProfile};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;

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
pub struct ProfileDatasetRequest {
    pub file_path: String,
    pub sample_size: Option<usize>,
}

pub async fn profile_dataset(
    State(profiler): State<Arc<DataProfiler>>,
    Json(payload): Json<ProfileDatasetRequest>,
) -> impl IntoResponse {
    // Check if file exists
    if !std::path::Path::new(&payload.file_path).exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("File not found".to_string())),
        );
    }

    match profiler.profile_file(&payload.file_path).await {
        Ok(profile) => (StatusCode::OK, Json(ApiResponse::success(profile))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to profile dataset: {}", e))),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<DataProfiler>> {
    use axum::routing::post;

    let profiler = Arc::new(DataProfiler::new());
    
    axum::Router::new()
        .route("/profile", post(profile_dataset))
        .with_state(profiler)
}
