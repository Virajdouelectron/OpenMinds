use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

mod api;
mod automl;
mod error;
mod models;
mod python;

use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize database
    let db = Arc::new(models::Database::new().await?);

    // Enable CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(health_check))
        .nest("/api", api::create_api_router())
        .layer(cors)
        .with_state(db);

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Server is running")
}
