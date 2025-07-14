use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{automl::{AutoMLRequest, AutoMLResponse, AutoMLService}, models::Database};

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

    fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }
    }
}

pub async fn start_automl_experiment(
    State(db): State<Arc<Database>>,
    Json(payload): Json<AutoMLRequest>,
) -> impl IntoResponse {
    let automl_service = AutoMLService::new(db.clone());
    
    match automl_service.start_automl(payload, None).await {
        Ok(response) => (StatusCode::ACCEPTED, Json(ApiResponse::success(response))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!("Failed to start AutoML experiment: {}", e))),
        ),
    }
}

pub async fn get_experiment_status(
    State(db): State<Arc<Database>>,
    Path(experiment_id): Path<String>,
) -> impl IntoResponse {
    let automl_service = AutoMLService::new(db);
    
    match automl_service.get_experiment_status(&experiment_id).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!("Failed to get experiment status: {}", e))),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<Database>> {
    use axum::routing::*;

    Router::new()
        .route("/", post(start_automl_experiment))
        .route("/:experiment_id", get(get_experiment_status))
}
