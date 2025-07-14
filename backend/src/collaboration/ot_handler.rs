use super::{
    message::{CollaborationMessage, OTOperation},
    ot::OTError,
    session::SessionManager,
    state::CollaborationState,
    error::CollaborationError,
};
use axum::{
    extract::ws::Message as WsMessage,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Handler for operational transform operations
pub struct OTHandler {
    state: Arc<RwLock<CollaborationState>>,
    sessions: Arc<SessionManager>,
}

impl OTHandler {
    /// Create a new OTHandler with the given state and session manager
    pub fn new(state: Arc<RwLock<CollaborationState>>, sessions: Arc<SessionManager>) -> Self {
        Self { state, sessions }
    }

    /// Handle an incoming operation from a client
    pub async fn handle_operation(
        &self,
        notebook_id: Uuid,
        user_id: Uuid,
        operation: OTOperation,
        version: u64,
    ) -> Result<(), CollaborationError> {
        // Transform the operation against any concurrent operations
        let transformed_op = self
            .state
            .read()
            .await
            .transform_operation(notebook_id, operation, version)
            .await?;

        // Apply the transformed operation
        let (content, new_version) = self
            .state
            .write()
            .await
            .apply_operation(notebook_id, transformed_op.clone(), user_id)
            .await?;

        // Broadcast the operation to all clients (including sender for confirmation)
        self.broadcast(
            notebook_id,
            &CollaborationMessage::OperationAck {
                user_id,
                notebook_id,
                operation: transformed_op,
                version: new_version,
            },
            &[],
        )
        .await
    }

    /// Handle a sync request from a client
    pub async fn handle_sync_request(
        &self,
        notebook_id: Uuid,
        user_id: Uuid,
        since_version: u64,
    ) -> Result<(), CollaborationError> {
        // Get current document state
        let (content, current_version) = self
            .state
            .read()
            .await
            .get_document(notebook_id)
            .await
            .ok_or_else(|| CollaborationError::NotebookNotFound)?;

        // Get current cursors
        let cursors = self.state.read().await.get_cursors(notebook_id).await?;

        // Send sync response to the requesting client
        let sessions = self.sessions.get_sessions(notebook_id).await;
        for session in sessions.into_iter().filter(|s| s.user_id == user_id) {
            if let Err(e) = session.sender.send(WsMessage::Text(
                serde_json::to_string(&CollaborationMessage::SyncResponse {
                    notebook_id,
                    content: content.clone(),
                    version: current_version,
                    operations: Vec::new(), // In a real implementation, include operations since 'since_version'
                    cursors: cursors.clone(),
                })?,
            )) {
                log::error!("Failed to send sync response: {}", e);
            }
        }

        Ok(())
    }

    /// Broadcast a message to all connected clients
    async fn broadcast(
        &self,
        notebook_id: Uuid,
        message: &CollaborationMessage,
        exclude_user_ids: &[Uuid],
    ) -> Result<(), CollaborationError> {
        let sessions = self.sessions.get_sessions(notebook_id).await;
        let message_str = serde_json::to_string(message)?;

        for session in sessions {
            if !exclude_user_ids.contains(&session.user_id) {
                if let Err(e) = session
                    .sender
                    .send(WsMessage::Text(message_str.clone()))
                    .map_err(Into::into)
                {
                    log::error!("Failed to send message to user {}: {}", session.user_id, e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collaboration::{
        message::{OTOperation, OTInsert, OTDelete},
        session::Session,
    };
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_handle_operation() {
        // Setup
        let state = Arc::new(RwLock::new(CollaborationState::new()));
        let sessions = Arc::new(SessionManager::new());
        let handler = OTHandler::new(state.clone(), sessions.clone());
        
        let notebook_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Create a test session
        let (tx, mut rx) = mpsc::unbounded_channel();
        sessions.create(notebook_id, user_id, tx).await.unwrap();
        
        // Create notebook
        state.write().await.create_notebook(notebook_id, "").await;
        
        // Test insert operation
        let insert_op = OTOperation::Insert(OTInsert {
            position: 0,
            text: "Hello".to_string(),
        });
        
        handler.handle_operation(notebook_id, user_id, insert_op, 0).await.unwrap();
        
        // Verify document content
        let (content, _) = state.read().await.get_document(notebook_id).await.unwrap();
        assert_eq!(content, "Hello");
        
        // Verify broadcast message
        if let Some(WsMessage::Text(msg)) = rx.recv().await {
            let msg: serde_json::Value = serde_json::from_str(&msg).unwrap();
            assert_eq!(msg["notebook_id"], notebook_id.to_string());
            assert_eq!(msg["user_id"], user_id.to_string());
            assert_eq!(msg["version"], 1);
        } else {
            panic!("Expected WebSocket message");
        }
    }
}
