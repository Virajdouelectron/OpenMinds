use axum::{
    extract::{Multipart, Path, State},
    Extension, Json,
};
use serde::Serialize;
use std::path::Path as StdPath;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{Database, Dataset},
};

#[derive(Debug, Serialize)]
pub struct DatasetResponse {
    id: String,
    name: String,
    size: i64,
    created_at: chrono::NaiveDateTime,
}

pub async fn upload_dataset(
    Extension(db): Extension<std::sync::Arc<Database>>,
    mut multipart: Multipart,
) -> Result<Json<DatasetResponse>> {
    let mut name = None;
    let mut file_data = None;

    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().unwrap_or_default().to_string();
        
        if field_name == "file" {
            let file_name = field.file_name()
                .map(ToString::to_string)
                .ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?;
                
            let data = field.bytes().await?;
            
            name = Some(file_name);
            file_data = Some(data);
        }
    }

    let file_name = name.ok_or_else(|| AppError::BadRequest("No file provided".to_string()))?;
    let file_data = file_data.ok_or_else(|| AppError::BadRequest("No file data".to_string()))?;
    
    // Create datasets directory if it doesn't exist
    let datasets_dir = StdPath::new("data/datasets");
    if !datasets_dir.exists() {
        tokio::fs::create_dir_all(datasets_dir).await?;
    }
    
    // Generate a unique filename
    let file_id = Uuid::new_v4();
    let file_ext = StdPath::new(&file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let new_file_name = format!("{}.{}", file_id, file_ext);
    let file_path = datasets_dir.join(&new_file_name);
    
    // Save the file
    tokio::fs::write(&file_path, &file_data).await?;
    
    // Get file size
    let metadata = tokio::fs::metadata(&file_path).await?;
    let size = metadata.len() as i64;
    
    // Save dataset info to database
    let id = file_id.to_string();
    let now = chrono::Utc::now().naive_utc();
    
    let dataset = sqlx::query_as!(
        Dataset,
        r#"
        INSERT INTO datasets (id, name, file_path, size, created_at)
        VALUES (?, ?, ?, ?, ?)
        RETURNING *
        "#,
        id,
        file_name,
        file_path.to_string_lossy(),
        size,
        now
    )
    .fetch_one(&*db.pool)
    .await?;
    
    Ok(Json(DatasetResponse {
        id: dataset.id,
        name: dataset.name,
        size: dataset.size,
        created_at: dataset.created_at,
    }))
}

pub async fn list_datasets(
    Extension(db): Extension<std::sync::Arc<Database>>,
) -> Result<Json<Vec<DatasetResponse>>> {
    let datasets = sqlx::query_as!(
        DatasetResponse,
        r#"
        SELECT id, name, size, created_at
        FROM datasets
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&*db.pool)
    .await?;

    Ok(Json(datasets))
}

#[derive(Debug, Serialize)]
pub struct DatasetPreview {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

pub async fn preview_dataset(
    Path(id): Path<Uuid>,
    Extension(db): Extension<std::sync::Arc<Database>>,
) -> Result<Json<DatasetPreview>> {
    let dataset = sqlx::query_as!(
        Dataset,
        r#"
        SELECT * FROM datasets
        WHERE id = ?
        "#,
        id.to_string()
    )
    .fetch_optional(&*db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Dataset not found".to_string()))?;
    
    // In a real implementation, we would:
    // 1. Read the file based on its type (CSV, Excel, etc.)
    // 2. Parse the first few rows
    // 3. Return the preview data
    
    // This is a simplified version that returns mock data
    Ok(Json(DatasetPreview {
        headers: vec!["Column 1".to_string(), "Column 2".to_string()],
        rows: vec![
            vec!["Value 1".to_string(), "Value 2".to_string()],
            vec!["Value 3".to_string(), "Value 4".to_string()],
        ],
    }))
}
