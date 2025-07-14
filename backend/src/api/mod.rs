pub mod ai;
pub mod automl;
pub mod collaboration;
pub mod datasets;
mod environment;
pub mod experiments;
pub mod model_builder;
pub mod notebooks;
pub mod profiler;

use axum::{
    Router,
    routing::get,
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
        Path,
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    models::Database,
    ai::{
        copilot::AICopilot,
        model_builder::ModelBuilder,
    },
    collaboration::{
        CollaborationServer,
        handler::CollaborationHandler,
        state::CollaborationState,
        session::SessionManager,
        error::CollaborationError,
    },
    data::profiler::DataProfiler,
    runtime::environment::EnvironmentManager,
};

pub fn create_api_router() -> Router {
    // Create the AI Copilot instance
    let ai_copilot = Arc::new(
        tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(AICopilot::new())
            .expect("Failed to initialize AI Copilot"),
    );

    // Create the Model Builder instance
    let model_builder = Arc::new(ModelBuilder::new());
    
    // Create the Data Profiler instance
    let data_profiler = Arc::new(DataProfiler::new());
    
    // Create the Environment Manager
    let environment_manager = Arc::new(Mutex::new(
        EnvironmentManager::new().expect("Failed to initialize environment manager")
    ));
    
    // Create collaboration components
    let collaboration_state = Arc::new(tokio::sync::RwLock::new(CollaborationState::new()));
    let session_manager = Arc::new(SessionManager::new());
    let collaboration_handler = Arc::new(CollaborationHandler::new(
        Arc::clone(&collaboration_state),
        Arc::clone(&session_manager),
    ));

    // Create a shared state that includes all services
    #[derive(Clone)]
    struct AppState {
        db: Arc<Database>,
        ai_copilot: Arc<AICopilot>,
        model_builder: Arc<ModelBuilder>,
        data_profiler: Arc<DataProfiler>,
        collaboration_handler: Arc<CollaborationHandler>,
    }

    let state = AppState {
        db: Arc::new(
            tokio::runtime::Runtime::new()
                .expect("Failed to create tokio runtime")
                .block_on(Database::new())
                .expect("Failed to initialize database"),
        ),
        ai_copilot,
        model_builder,
        data_profiler,
        collaboration_handler,
    };

    // Create API router with all routes
    let api_router = Router::new()
        .nest("/ai", ai::create_router())
        .nest("/model-builder", model_builder::create_router())
        .nest("/profiler", profiler::create_router())
        .nest("/environments", environment::create_router())
        .nest("/datasets", datasets::create_router())
        .nest("/experiments", experiments::create_router())
        .nest("/automl", automl::create_router())
        .nest("/notebooks", notebooks::create_router())
        .nest("/collaboration", collaboration::create_router())
        .with_state(state);
    
    // Create WebSocket router for real-time collaboration
    let ws_router = Router::new()
        .route("/ws/:notebook_id", get(handle_websocket))
        .with_state(collaboration_handler);
    
    // Add a health check endpoint
    let app = Router::new()
        .nest("/api", api_router)
        .nest("/ws", ws_router)
        .route("/health", get(|| async { "OK" }));
        
    app
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    Path(notebook_id): Path<Uuid>,
    State(handler): State<Arc<CollaborationHandler>>,
) -> impl IntoResponse {
    // In a real app, you'd get the user ID from authentication
    let user_id = Uuid::new_v4();
    
    ws.on_upgrade(move |socket| async move {
        if let Err(e) = handler.handle_connection(socket, notebook_id, user_id).await {
            log::error!("WebSocket connection error: {}", e);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_api_router();
        let response = app
            .oneshot(Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
