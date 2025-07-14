use crate::ai::model_builder::{ModelBuilder, ModelGraph, GeneratedModel};
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
pub struct GenerateModelRequest {
    pub graph: ModelGraph,
    pub framework: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GenerateModelResponse {
    pub model_id: String,
    pub code: String,
    pub requirements: Vec<String>,
    pub summary: String,
}

pub async fn generate_model(
    State(builder): State<Arc<ModelBuilder>>,
    Json(payload): Json<GenerateModelRequest>,
) -> impl IntoResponse {
    match builder.generate_model(&payload.graph) {
        Ok(generated) => {
            let response = GenerateModelResponse {
                model_id: uuid::Uuid::new_v4().to_string(),
                code: generated.code,
                requirements: generated.requirements,
                summary: generated.model_summary,
            };
            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(format!("Failed to generate model: {}", e))),
        ),
    }
}

pub fn create_router() -> axum::Router<Arc<ModelBuilder>> {
    use axum::routing::post;

    let builder = Arc::new(ModelBuilder::new());
    
    axum::Router::new()
        .route("/generate", post(generate_model))
        .with_state(builder)
}
