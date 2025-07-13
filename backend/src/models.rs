use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Database {
    pool: Arc<sqlx::SqlitePool>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notebook {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cell {
    pub id: String,
    pub notebook_id: String,
    pub cell_type: String, // 'code' or 'markdown'
    pub content: String,
    pub output: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub size: i64,
    pub created_at: chrono::NaiveDateTime,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Create data directory if it doesn't exist
        tokio::fs::create_dir_all("data").await?;
        
        let database_url = "sqlite:data/openmind.db";
        let pool = SqlitePoolOptions::new()
            .connect(database_url)
            .await?;
            
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
            
        Ok(Self { pool: Arc::new(pool) })
    }
    
    // Notebook CRUD operations
    pub async fn create_notebook(&self, name: &str) -> Result<Notebook, sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();
        
        sqlx::query_as!(
            Notebook,
            r#"
            INSERT INTO notebooks (id, name, created_at, updated_at)
            VALUES (?, ?, ?, ?)
            RETURNING *
            "#,
            id,
            name,
            now,
            now
        )
        .fetch_one(&*self.pool)
        .await
    }
    
    // Add other CRUD operations for notebooks, cells, and datasets
}

// Create migrations directory and initial migration
#[cfg(not(feature = "migrate"))]
mod migrations {
    use sqlx::migrate::Migrator;
    use std::path::Path;
    
    pub fn get_migrator() -> Migrator {
        sqlx::migrate!("migrations")
    }
}
