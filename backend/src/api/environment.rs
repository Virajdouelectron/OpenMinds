use crate::runtime::environment::{EnvironmentManager, RuntimeEnvironment};
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
pub struct SetDefaultEnvironmentRequest {
    pub environment_id: String,
}

pub async fn list_environments(
    State(manager): State<Arc<EnvironmentManager>>,
) -> impl IntoResponse {
    let environments = manager.list_environments().to_vec();
    (StatusCode::OK, Json(ApiResponse::success(environments)))
}

pub async fn get_environment(
    State(manager): State<Arc<EnvironmentManager>>,
    axum::extract::Path(environment_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    match manager.get_environment(&environment_id) {
        Some(env) => (StatusCode::OK, Json(ApiResponse::success(env))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("Environment not found".to_string())),
        ),
    }
}

pub async fn get_default_environment(
    State(manager): State<Arc<EnvironmentManager>>,
) -> impl IntoResponse {
    match manager.get_default_environment() {
        Some(env) => (StatusCode::OK, Json(ApiResponse::success(env))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("No default environment set".to_string())),
        ),
    }
}

pub async fn set_default_environment(
    State(manager): State<Arc<tokio::sync::Mutex<EnvironmentManager>>>,
    Json(payload): Json<SetDefaultEnvironmentRequest>,
) -> impl IntoResponse {
    let mut manager = manager.lock().await;
    match manager.set_default_environment(&payload.environment_id) {
        Ok(_) => (StatusCode::OK, Json(ApiResponse::success(()))),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(e.to_string())),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<tokio::sync::Mutex<EnvironmentManager>>> {
    use axum::routing::{get, post};

    let manager = Arc::new(tokio::sync::Mutex::new(
        EnvironmentManager::new().expect("Failed to initialize environment manager")
    ));
    
    axum::Router::new()
        .route("/environments", get(list_environments))
        .route("/environments/default", get(get_default_environment))
        .route("/environments/default", post(set_default_environment))
        .route("/environments/:environment_id", get(get_environment))
        .with_state(manager)
}
