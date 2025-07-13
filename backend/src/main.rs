use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::{net::SocketAddr, sync::Arc};

mod api;
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

    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/notebooks", get(api::notebooks::list_notebooks))
        .route("/api/notebooks", post(api::notebooks::create_notebook))
        .route("/api/notebooks/:id", get(api::notebooks::get_notebook))
        .route("/api/notebooks/:id/cells", post(api::notebooks::add_cell))
        .route(
            "/api/notebooks/:notebook_id/cells/:cell_id/execute",
            post(api::notebooks::execute_cell),
        )
        .route("/api/datasets", post(api::datasets::upload_dataset))
        .route("/api/datasets", get(api::datasets::list_datasets))
        .layer(Extension(db));

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
