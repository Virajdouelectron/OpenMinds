use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::Result,
    models::{Cell, Database, Notebook},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotebookRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCellRequest {
    pub cell_type: String,
    pub content: String,
}

pub async fn list_notebooks(
    Extension(db): Extension<std::sync::Arc<Database>>,
) -> Result<Json<Vec<Notebook>>> {
    let notebooks = sqlx::query_as!(
        Notebook,
        r#"
        SELECT * FROM notebooks
        ORDER BY updated_at DESC
        "#
    )
    .fetch_all(&*db.pool)
    .await?;

    Ok(Json(notebooks))
}

pub async fn create_notebook(
    Extension(db): Extension<std::sync::Arc<Database>>,
    Json(payload): Json<CreateNotebookRequest>,
) -> Result<Json<Notebook>> {
    let notebook = db.create_notebook(&payload.name).await?;
    Ok(Json(notebook))
}

pub async fn get_notebook(
    Path(id): Path<Uuid>,
    Extension(db): Extension<std::sync::Arc<Database>>,
) -> Result<Json<(Notebook, Vec<Cell>)>> {
    let notebook = sqlx::query_as!(
        Notebook,
        r#"
        SELECT * FROM notebooks
        WHERE id = ?
        "#,
        id.to_string()
    )
    .fetch_one(&*db.pool)
    .await?;

    let cells = sqlx::query_as!(
        Cell,
        r#"
        SELECT * FROM cells
        WHERE notebook_id = ?
        ORDER BY created_at
        "#,
        id.to_string()
    )
    .fetch_all(&*db.pool)
    .await?;

    Ok(Json((notebook, cells)))
}

pub async fn add_cell(
    Path(notebook_id): Path<Uuid>,
    Extension(db): Extension<std::sync::Arc<Database>>,
    Json(payload): Json<CreateCellRequest>,
) -> Result<Json<Cell>> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let cell = sqlx::query_as!(
        Cell,
        r#"
        INSERT INTO cells (id, notebook_id, cell_type, content, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
        id,
        notebook_id.to_string(),
        payload.cell_type,
        payload.content,
        now,
        now
    )
    .fetch_one(&*db.pool)
    .await?;

    // Update notebook's updated_at
    sqlx::query!(
        "UPDATE notebooks SET updated_at = ? WHERE id = ?",
        now,
        notebook_id.to_string()
    )
    .execute(&*db.pool)
    .await?;

    Ok(Json(cell))
}

pub async fn execute_cell(
    Path((notebook_id, cell_id)): Path<(Uuid, Uuid)>,
    Extension(db): Extension<std::sync::Arc<Database>>,
) -> Result<Json<Cell>> {
    // In a real implementation, we would:
    // 1. Get the cell content
    // 2. Execute it using the Python executor
    // 3. Save the output
    // 4. Return the updated cell
    
    // This is a simplified version that just updates the cell with a mock output
    let now = chrono::Utc::now().naive_utc();
    
    let cell = sqlx::query_as!(
        Cell,
        r#"
        UPDATE cells 
        SET output = ?, updated_at = ?
        WHERE id = ? AND notebook_id = ?
        RETURNING *
        "#,
        Some("Output will be displayed here".to_string()),
        now,
        cell_id.to_string(),
        notebook_id.to_string()
    )
    .fetch_one(&*db.pool)
    .await?;

    // Update notebook's updated_at
    sqlx::query!(
        "UPDATE notebooks SET updated_at = ? WHERE id = ?",
        now,
        notebook_id.to_string()
    )
    .execute(&*db.pool)
    .await?;

    Ok(Json(cell))
}
