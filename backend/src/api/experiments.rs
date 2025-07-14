use crate::models::Database;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
pub struct CreateExperimentRequest {
    pub name: String,
    pub notebook_id: Option<String>,
    pub dataset_id: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExperimentMetricsRequest {
    pub metrics: Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExperimentStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct AddExperimentLogRequest {
    pub level: String,
    pub message: String,
}

pub async fn create_experiment(
    State(db): State<Arc<Database>>,
    Json(payload): Json<CreateExperimentRequest>,
) -> impl IntoResponse {
    match db
        .create_experiment(
            &payload.name,
            payload.notebook_id.as_deref(),
            payload.dataset_id.as_deref(),
            payload.parameters,
        )
        .await
    {
        Ok(experiment) => (StatusCode::CREATED, Json(ApiResponse::success(experiment))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!("Failed to create experiment: {}", e))),
        ),
    }
}

pub async fn get_experiment(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db.get_experiment(&id).await {
        Ok(Some(experiment)) => (StatusCode::OK, Json(ApiResponse::success(experiment))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Experiment not found")),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!("Failed to get experiment: {}", e))),
        ),
    }
}

pub async fn update_experiment_metrics(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateExperimentMetricsRequest>,
) -> impl IntoResponse {
    match db.update_experiment_metrics(&id, payload.metrics).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Metrics updated successfully")),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!(
                "Failed to update metrics: {}",
                e
            ))),
        ),
    }
}

pub async fn update_experiment_status(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateExperimentStatusRequest>,
) -> impl IntoResponse {
    match db.update_experiment_status(&id, &payload.status).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Status updated successfully")),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!(
                "Failed to update status: {}",
                e
            ))),
        ),
    }
}

pub async fn add_experiment_log(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Json(payload): Json<AddExperimentLogRequest>,
) -> impl IntoResponse {
    match db
        .add_experiment_log(&id, &payload.level, &payload.message)
        .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(ApiResponse::success("Log added successfully")),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!("Failed to add log: {}", e))),
        ),
    }
}

pub async fn get_experiment_logs(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db.get_experiment_logs(&id, None).await {
        Ok(logs) => (StatusCode::OK, Json(ApiResponse::success(logs))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(&format!(
                "Failed to get experiment logs: {}",
                e
            ))),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<Database>> {
    use axum::routing::*;

    Router::new()
        .route("/", post(create_experiment))
        .route(
            "/:id",
            get(get_experiment)
                .patch(update_experiment_metrics)
                .put(update_experiment_status),
        )
        .route(
            "/:id/logs",
            get(get_experiment_logs).post(add_experiment_log),
        )
}
