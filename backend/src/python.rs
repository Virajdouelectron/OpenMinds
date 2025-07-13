use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;
use tokio::process::Command;

#[derive(Debug)]
pub struct PythonExecutor {
    python_path: String,
}

impl PythonExecutor {
    pub fn new() -> Self {
        // Default to 'python3', can be overridden in config
        Self {
            python_path: "python3".to_string(),
        }
    }

    pub async fn execute_code(&self, code: &str) -> Result<String, String> {
        let output = Command::new(&self.python_path)
            .arg("-c")
            .arg(code)
            .output()
            .await
            .map_err(|e| format!("Failed to execute Python: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Python execution error: {}", stderr))
        }
    }

    pub async fn execute_notebook_cell(
        &self,
        notebook_path: &str,
        cell_index: usize,
    ) -> Result<String, String> {
        let script = format!(
            r#"
import json
from jupyter_client import KernelManager
import time

# Create a kernel manager
km = KernelManager()
km.start_kernel()

# Get the client
kc = km.client()
kc.start_channels()

# Load the notebook
with open('{}', 'r') as f:
    notebook = json.load(f)

# Get the cell to execute
cell = notebook['cells'][{}]

# Execute the cell
msg_id = kc.execute(cell['source'])

# Get the result
outputs = []
while True:
    try:
        msg = kc.get_iopub_msg(timeout=1)
        if msg['parent_header'].get('msg_id') == msg_id:
            if msg['msg_type'] == 'status' and msg['content']['execution_state'] == 'idle':
                break
            if 'data' in msg['content']:
                outputs.append(msg['content']['data']['text/plain'])
    except Exception as e:
        break

# Clean up
kc.stop_channels()
km.shutdown_kernel()

# Return the outputs as a string
print('\n'.join(outputs))
"#,
            notebook_path, cell_index
        );

        self.execute_code(&script).await
    }

    pub async fn train_model(
        &self,
        script_path: &str,
        dataset_path: &str,
        output_path: &str,
    ) -> Result<String, String> {
        let output = Command::new(&self.python_path)
            .arg(script_path)
            .arg("--dataset")
            .arg(dataset_path)
            .arg("--output")
            .arg(output_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute training script: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Training script error: {}", stderr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_code() {
        let executor = PythonExecutor::new();
        let result = executor.execute_code("print(2 + 2)").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "4");
    }
}
