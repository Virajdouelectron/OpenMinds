use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    collaboration::{
        CollaborationHandler,
        state::CollaborationState,
        session::SessionManager,
        error::CollaborationError,
    },
    models::Database,
};

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
pub struct CreateSessionRequest {
    pub notebook_id: Uuid,
    pub user_id: Uuid,
}

pub async fn create_session(
    State(handler): State<Arc<CollaborationHandler>>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    // In a real app, you'd validate the user has access to the notebook
    Ok(Json(ApiResponse::success(())))
}

pub async fn get_notebook_state(
    Path(notebook_id): Path<Uuid>,
    State(handler): State<Arc<CollaborationHandler>>,
) -> impl IntoResponse {
    match handler.state.read().await.get_content(notebook_id).await {
        Some((content, version)) => (
            StatusCode::OK,
            Json(ApiResponse::success(serde_json::json!({ "content": content, "version": version }))),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("Notebook not found".to_string())),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<CollaborationHandler>> {
    use axum::routing::{get, post};

    let state = Arc::new(tokio::sync::RwLock::new(CollaborationState::new()));
    let sessions = Arc::new(SessionManager::new());
    let handler = Arc::new(CollaborationHandler::new(state, sessions));
    
    axum::Router::new()
        .route("/sessions", post(create_session))
        .route("/notebooks/:notebook_id/state", get(get_notebook_state))
        .with_state(handler)
}
