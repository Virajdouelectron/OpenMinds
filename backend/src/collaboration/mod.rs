pub mod session;
pub mod state;
pub mod handler;
pub mod message;
pub mod error;

use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message as WsMessage}, State},
    response::Response,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::models::Database;

use self::{
    error::CollaborationError,
    handler::CollaborationHandler,
    message::CollaborationMessage,
    session::SessionManager,
    state::CollaborationState,
};

pub type SharedState = Arc<RwLock<CollaborationState>>;

pub struct CollaborationServer {
    state: SharedState,
    sessions: Arc<SessionManager>,
    db: Arc<Database>,
}

impl CollaborationServer {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            state: Arc::new(RwLock::new(CollaborationState::new())),
            sessions: Arc::new(SessionManager::new()),
            db,
        }
    }

    pub async fn handle_connection(
        &self,
        ws: WebSocket,
        notebook_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), CollaborationError> {
        // Register the new session
        let (mut ws_sender, mut ws_receiver) = ws.split();
        let session_id = self.sessions.create(notebook_id, user_id, ws_sender).await?;

        // Notify other users about the new participant
        self.broadcast(
            notebook_id,
            &CollaborationMessage::UserJoined {
                user_id,
                notebook_id,
            },
        )
        .await?;

        // Main message handling loop
        while let Some(Ok(message)) = ws_receiver.next().await {
            match message {
                WsMessage::Text(text) => {
                    if let Ok(msg) = serde_json::from_str::<CollaborationMessage>(&text) {
                        self.handle_message(notebook_id, user_id, msg).await?;
                    }
                }
                WsMessage::Close(_) => {
                    break;
                }
                _ => {}
            }
        }

        // Clean up on disconnect
        self.sessions.remove(&session_id).await;
        self.broadcast(
            notebook_id,
            &CollaborationMessage::UserLeft { user_id, notebook_id },
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
                self.broadcast(
                    notebook_id,
                    &CollaborationMessage::CursorMove {
                        user_id,
                        notebook_id,
                        position,
                    },
                )
                .await
            }
            CollaborationMessage::ContentUpdate { content, .. } => {
                // Update the shared state
                let mut state = self.state.write().await;
                state.update_content(notebook_id, content.clone()).await?;
                
                // Broadcast the update to all connected clients except the sender
                self.broadcast_except(
                    notebook_id,
                    &CollaborationMessage::ContentUpdate {
                        user_id,
                        notebook_id,
                        content,
                    },
                    &[user_id],
                )
                .await
            }
            _ => Ok(()),
        }
    }

    async fn broadcast(
        &self,
        notebook_id: Uuid,
        message: &CollaborationMessage,
    ) -> Result<(), CollaborationError> {
        self.broadcast_except(notebook_id, message, &[]).await
    }

    async fn broadcast_except(
        &self,
        notebook_id: Uuid,
        message: &CollaborationMessage,
        exclude_user_ids: &[Uuid],
    ) -> Result<(), CollaborationError> {
        let sessions = self.sessions.get_sessions(notebook_id).await;
        let message_str = serde_json::to_string(message)?;

        for session in sessions {
            if !exclude_user_ids.contains(&session.user_id) {
                if let Err(e) = session.sender.send(WsMessage::Text(message_str.clone())).await {
                    log::error!("Failed to send message to user {}: {}", session.user_id, e);
                }
            }
        }

        Ok(())
    }
}

pub fn create_collaboration_router(db: Arc<Database>) -> Router {
    let collaboration = CollaborationServer::new(db);
    
    Router::new()
        .route("/ws/:notebook_id", get(handle_websocket))
        .with_state(Arc::new(Mutex::new(collaboration)))
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    Path(notebook_id): Path<Uuid>,
    State(collaboration): State<Arc<Mutex<CollaborationServer>>>,
    // In a real app, you'd get this from auth middleware
    // user_id: UserId,
) -> Response {
    // For demo purposes, generate a random user ID
    let user_id = Uuid::new_v4();
    
    ws.on_upgrade(move |socket| async move {
        if let Err(e) = collaboration
            .lock()
            .await
            .handle_connection(socket, notebook_id, user_id)
            .await
        {
            log::error!("WebSocket error: {}", e);
        }
    })
}
