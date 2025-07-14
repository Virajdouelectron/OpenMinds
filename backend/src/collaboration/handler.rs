use super::{
    message::CollaborationMessage,
    session::SessionManager,
    state::CollaborationState,
    error::CollaborationError,
};
use axum::{
    extract::ws::{Message as WsMessage, WebSocket},
    extract::State,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct CollaborationHandler {
    state: Arc<tokio::sync::RwLock<CollaborationState>>,
    sessions: Arc<SessionManager>,
}

impl CollaborationHandler {
    pub fn new(state: Arc<tokio::sync::RwLock<CollaborationState>>, sessions: Arc<SessionManager>) -> Self {
        Self { state, sessions }
    }

    pub async fn handle_connection(
        &self,
        mut socket: WebSocket,
        notebook_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), CollaborationError> {
        // Create a channel for sending messages to this client
        let (mut ws_sender, mut ws_receiver) = socket.split();
        
        // Create a channel for the session
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        
        // Spawn a task to send messages to the client
        let send_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = ws_sender.send(message).await {
                    log::error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
        });

        // Add the session
        let session_id = self.sessions.create(notebook_id, user_id, tx).await?;
        
        // Notify other users about the new participant
        self.broadcast(
            notebook_id,
            &CollaborationMessage::UserJoined {
                user_id,
                notebook_id,
            },
            &[user_id], // Exclude self
        )
        .await?;

        // Send current state to the new user
        if let Some((content, version)) = self.state.read().await.get_content(notebook_id).await {
            let message = CollaborationMessage::SyncResponse {
                notebook_id,
                content,
                version,
                operations: Vec::new(),
            };
            
            if let Err(e) = tx.send(WsMessage::Text(serde_json::to_string(&message)?)) {
                log::error!("Failed to send initial state: {}", e);
            }
        }

        // Handle incoming messages
        while let Some(Ok(message)) = ws_receiver.next().await {
            match message {
                WsMessage::Text(text) => {
                    if let Ok(message) = serde_json::from_str::<CollaborationMessage>(&text) {
                        self.handle_message(notebook_id, user_id, message).await?;
                    }
                }
                WsMessage::Close(_) => {
                    break;
                }
                _ => {}
            }
        }

        // Clean up
        send_task.abort();
        self.sessions.remove(&session_id).await;
        
        // Notify other users about the departure
        self.broadcast(
            notebook_id,
            &CollaborationMessage::UserLeft {
                user_id,
                notebook_id,
            },
            &[],
        )
        .await?;

        Ok(())
    }

    async fn handle_message(
        &self,
        notebook_id: Uuid,
        user_id: Uuid,
        message: CollaborationMessage,
    ) -> Result<(), CollaborationError> {
        match message {
            CollaborationMessage::CursorMove { position, .. } => {
                self.state
                    .write()
                    .await
                    .update_cursor(
                        user_id,
                        super::state::CursorPosition {
                            notebook_id,
                            line: position.line,
                            column: position.column,
                            selection_start: position.selection_start,
                        },
                    )
                    .await?;

                self.broadcast(
                    notebook_id,
                    &CollaborationMessage::CursorMove {
                        user_id,
                        notebook_id,
                        position,
                    },
                    &[user_id], // Exclude self
                )
                .await
            }
            CollaborationMessage::ContentUpdate { content, .. } => {
                let version = self
                    .state
                    .write()
                    .await
                    .update_content(notebook_id, content.clone())
                    .await?;

                self.broadcast(
                    notebook_id,
                    &CollaborationMessage::SyncResponse {
                        notebook_id,
                        content,
                        version,
                        operations: Vec::new(),
                    },
                    &[user_id], // Exclude self
                )
                .await
            }
            CollaborationMessage::Operation { operation, .. } => {
                // In a real implementation, you would apply the operation to the document
                // and broadcast the transformed operation to all clients
                self.broadcast(
                    notebook_id,
                    &CollaborationMessage::Operation {
                        user_id,
                        notebook_id,
                        operation,
                    },
                    &[user_id], // Exclude self
                )
                .await
            }
            _ => {}
        }
        Ok(())
    }

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
