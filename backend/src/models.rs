use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use chrono::NaiveDateTime;

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
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub notebook_id: Option<String>,
    pub status: String,
    pub metrics: Option<String>,
    pub parameters: Option<String>,
    pub dataset_id: Option<String>,
    pub model_path: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExperimentLog {
    pub id: i64,
    pub experiment_id: String,
    pub timestamp: NaiveDateTime,
    pub level: String,
    pub message: String,
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
    
    // Experiment CRUD operations
    pub async fn create_experiment(
        &self,
        name: &str,
        notebook_id: Option<&str>,
        dataset_id: Option<&str>,
        parameters: Option<Value>,
    ) -> Result<Experiment, sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();
        let params_str = parameters.map(|p| p.to_string());

        sqlx::query_as!(
            Experiment,
            r#"
            INSERT INTO experiments (
                id, name, notebook_id, dataset_id, parameters, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
            id,
            name,
            notebook_id,
            dataset_id,
            params_str,
            now,
            now
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_experiment(&self, id: &str) -> Result<Option<Experiment>, sqlx::Error> {
        sqlx::query_as!(
            Experiment,
            "SELECT * FROM experiments WHERE id = ?",
            id
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn update_experiment_metrics(
        &self,
        id: &str,
        metrics: Value,
    ) -> Result<(), sqlx::Error> {
        let metrics_str = metrics.to_string();
        sqlx::query!(
            "UPDATE experiments SET metrics = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            metrics_str,
            id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_experiment_status(
        &self,
        id: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE experiments SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            status,
            id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_experiment_log(
        &self,
        experiment_id: &str,
        level: &str,
        message: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO experiment_logs (experiment_id, level, message) VALUES (?, ?, ?)",
            experiment_id,
            level,
            message
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_experiment_logs(
        &self,
        experiment_id: &str,
        limit: Option<i64>,
    ) -> Result<Vec<ExperimentLog>, sqlx::Error> {
        let limit = limit.unwrap_or(100);
        sqlx::query_as!(
            ExperimentLog,
            "SELECT * FROM experiment_logs WHERE experiment_id = ? ORDER BY timestamp DESC LIMIT ?",
            experiment_id,
            limit
        )
        .fetch_all(&*self.pool)
        .await
    }
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
