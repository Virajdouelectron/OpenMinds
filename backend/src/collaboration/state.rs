use crate::collaboration::ot::{OTEngine, OTOperation, OTError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct NotebookState {
    pub engine: OTEngine,
    pub cursors: HashMap<Uuid, CursorPosition>,
}

impl NotebookState {
    pub fn new(initial_content: &str) -> Self {
        Self {
            engine: OTEngine::new(initial_content),
            cursors: HashMap::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct CollaborationState {
    // Maps notebook_id to its state (OT engine and cursors)
    notebooks: RwLock<HashMap<Uuid, NotebookState>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CursorPosition {
    pub notebook_id: Uuid,
    pub line: u32,
    pub column: u32,
    pub selection_start: Option<(u32, u32)>,
}

impl CollaborationState {
    pub fn new() -> Self {
        Self {
            notebooks: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create_notebook(&self, notebook_id: Uuid, initial_content: &str) {
        self.notebooks
            .write()
            .await
            .insert(notebook_id, NotebookState::new(initial_content));
    }

    pub async fn apply_operation(
        &self,
        notebook_id: Uuid,
        operation: OTOperation,
        client_id: Uuid,
    ) -> Result<(String, u64), CollaborationError> {
        let mut notebooks = self.notebooks.write().await;
        let notebook = notebooks
            .get_mut(&notebook_id)
            .ok_or_else(|| CollaborationError::NotebookNotFound)?;

        // Apply the operation to the OT engine
        let version = notebook.engine.apply_operation(operation, client_id)?;
        let content = notebook.engine.get_document().to_string();

        Ok((content, version))
    }

    pub async fn transform_operation(
        &self,
        notebook_id: Uuid,
        operation: OTOperation,
        since_version: u64,
    ) -> Result<OTOperation, CollaborationError> {
        let notebooks = self.notebooks.read().await;
        let notebook = notebooks
            .get(&notebook_id)
            .ok_or_else(|| CollaborationError::NotebookNotFound)?;

        notebook
            .engine
            .transform_operation(&operation, since_version)
            .map_err(Into::into)
    }

    pub async fn get_document(&self, notebook_id: Uuid) -> Option<(String, u64)> {
        let notebooks = self.notebooks.read().await;
        notebooks
            .get(¬ebook_id)
            .map(|notebook| (notebook.engine.get_document().to_string(), notebook.engine.get_version()))
    }

    pub async fn update_cursor(
        &self,
        notebook_id: Uuid,
        user_id: Uuid,
        position: CursorPosition,
    ) -> Result<(), CollaborationError> {
        let mut notebooks = self.notebooks.write().await;
        let notebook = notebooks
            .get_mut(¬ebook_id)
            .ok_or_else(|| CollaborationError::NotebookNotFound)?;

        notebook.cursors.insert(user_id, position);
        Ok(())
    }

    pub async fn get_cursors(
        &self,
        notebook_id: Uuid,
    ) -> Result<HashMap<Uuid, CursorPosition>, CollaborationError> {
        let notebooks = self.notebooks.read().await;
        let notebook = notebooks
            .get(¬ebook_id)
            .ok_or_else(|| CollaborationError::NotebookNotFound)?;

        Ok(notebook.cursors.clone())
    }

    pub async fn remove_user(
        &self,
        notebook_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), CollaborationError> {
        let mut notebooks = self.notebooks.write().await;
        if let Some(notebook) = notebooks.get_mut(¬ebook_id) {
            notebook.cursors.remove(&user_id);
            Ok(())
        } else {
            Err(CollaborationError::NotebookNotFound)
        }
    }
}
