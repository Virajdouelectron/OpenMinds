use crate::models::Database;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
};
use tempfile::NamedTempFile;
use tokio::fs;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoMLRequest {
    pub dataset_id: String,
    pub target_column: String,
    pub task_type: Option<String>,
    pub test_size: Option<f64>,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoMLResponse {
    pub experiment_id: String,
    pub status: String,
    pub metrics: Option<serde_json::Value>,
    pub model_path: Option<String>,
    pub results_path: Option<String>,
    pub error: Option<String>,
}

pub struct AutoMLService {
    db: Arc<Database>,
    python_path: PathBuf,
    automl_script_path: PathBuf,
}

impl AutoMLService {
    pub fn new(db: Arc<Database>) -> Self {
        // Get the path to the Python interpreter (use 'python3' by default)
        let python_path = std::env::var("PYTHON_PATH").unwrap_or_else(|_| "python3".to_string());
        
        // Get the path to the AutoML script
        let automl_script_path = Path::new("python/automl/service.py").to_path_buf();
        
        Self {
            db,
            python_path: PathBuf::from(python_path),
            automl_script_path,
        }
    }

    pub async fn start_automl(
        &self,
        request: AutoMLRequest,
        notebook_id: Option<&str>,
    ) -> Result<AutoMLResponse> {
        // Generate a unique experiment ID
        let experiment_id = Uuid::new_v4().to_string();
        
        // Get dataset path from the database
        let dataset = self
            .db
            .get_dataset(&request.dataset_id)
            .await
            .context("Failed to get dataset")?;
            
        if dataset.is_none() {
            return Err(anyhow::anyhow!("Dataset not found"));
        }
        let dataset = dataset.unwrap();
        
        // Create a temporary file for the dataset if needed
        let dataset_path = Path::new(&dataset.file_path);
        
        // Start the AutoML process in the background
        let python_path = self.python_path.clone();
        let automl_script_path = self.automl_script_path.clone();
        let experiment_id_clone = experiment_id.clone();
        let target_column = request.target_column.clone();
        let task_type = request.task_type.clone();
        let test_size = request.test_size.unwrap_or(0.2);
        
        // Spawn a new thread to run the AutoML process
        tokio::task::spawn_blocking(move || {
            // Build the command to run the AutoML script
            let mut cmd = Command::new(python_path);
            
            cmd.arg(automl_script_path)
                .arg("--data").arg(dataset_path)
                .arg("--target").arg(target_column)
                .arg("--experiment-id").arg(experiment_id_clone)
                .arg("--test-size").arg(test_size.to_string());
                
            if let Some(task_type) = task_type {
                cmd.arg("--task-type").arg(task_type);
            }
            
            // Set up output and error logging
            let output = cmd
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .expect("Failed to execute AutoML process");
                
            // Log the output and error
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("AutoML process failed: {}", stderr);
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                log::info!("AutoML process output: {}", stdout);
            }
        });
        
        // Return immediately with the experiment ID
        Ok(AutoMLResponse {
            experiment_id,
            status: "started".to_string(),
            metrics: None,
            model_path: None,
            results_path: None,
            error: None,
        })
    }
    
    pub async fn get_experiment_status(&self, experiment_id: &str) -> Result<AutoMLResponse> {
        // In a real implementation, this would check the status of a running experiment
        // For now, we'll just return a placeholder response
        Ok(AutoMLResponse {
            experiment_id: experiment_id.to_string(),
            status: "running".to_string(),
            metrics: None,
            model_path: None,
            results_path: None,
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_start_automl() {
        // This is a basic test that just verifies the function can be called
        // In a real test, you would set up a test database and test dataset
        let db = Database::new().await.unwrap();
        let automl_service = AutoMLService::new(Arc::new(db));
        
        let request = AutoMLRequest {
            dataset_id: "test-dataset".to_string(),
            target_column: "target".to_string(),
            task_type: Some("classification".to_string()),
            test_size: Some(0.2),
            parameters: None,
        };
        
        let response = automl_service.start_automl(request, None).await;
        assert!(response.is_ok());
    }
}
