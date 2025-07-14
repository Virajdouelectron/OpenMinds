use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{
    mpsc,
    RwLock,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notebook_id: Uuid,
    pub sender: mpsc::UnboundedSender<axum::extract::ws::Message>,
}

#[derive(Default, Debug)]
pub struct SessionManager {
    sessions: RwLock<HashMap<Uuid, Arc<Session>>>,
    user_sessions: RwLock<HashMap<Uuid, Vec<Uuid>>>,
    notebook_sessions: RwLock<HashMap<Uuid, Vec<Uuid>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            user_sessions: RwLock::new(HashMap::new()),
            notebook_sessions: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create(
        &self,
        notebook_id: Uuid,
        user_id: Uuid,
        sender: mpsc::UnboundedSender<axum::extract::ws::Message>,
    ) -> Result<Uuid, CollaborationError> {
        let session_id = Uuid::new_v4();
        let session = Arc::new(Session {
            id: session_id,
            user_id,
            notebook_id,
            sender,
        });

        // Add to sessions
        self.sessions.write().await.insert(session_id, session.clone());

        // Update user sessions
        self.user_sessions
            .write()
            .await
            .entry(user_id)
            .or_default()
            .push(session_id);

        // Update notebook sessions
        self.notebook_sessions
            .write()
            .await
            .entry(notebook_id)
            .or_default()
            .push(session_id);

        Ok(session_id)
    }

    pub async fn remove(&self, session_id: &Uuid) -> Option<Arc<Session>> {
        let session = self.sessions.write().await.remove(session_id)?;

        // Remove from user sessions
        if let Some(sessions) = self.user_sessions.write().await.get_mut(&session.user_id) {
            sessions.retain(|id| id != session_id);
            if sessions.is_empty() {
                self.user_sessions.write().await.remove(&session.user_id);
            }
        }

        // Remove from notebook sessions
        if let Some(sessions) = self.notebook_sessions.write().await.get_mut(&session.notebook_id) {
            sessions.retain(|id| id != session_id);
            if sessions.is_empty() {
                self.notebook_sessions.write().await.remove(&session.notebook_id);
            }
        }

        Some(session)
    }

    pub async fn get_sessions(&self, notebook_id: Uuid) -> Vec<Arc<Session>> {
        let notebook_sessions = self.notebook_sessions.read().await;
        let session_ids = match notebook_sessions.get(&notebook_id) {
            Some(ids) => ids,
            None => return Vec::new(),
        };

        let sessions = self.sessions.read().await;
        session_ids
            .iter()
            .filter_map(|id| sessions.get(id).cloned())
            .collect()
    }

    pub async fn get_user_sessions(&self, user_id: Uuid) -> Vec<Arc<Session>> {
        let user_sessions = self.user_sessions.read().await;
        let session_ids = match user_sessions.get(&user_id) {
            Some(ids) => ids,
            None => return Vec::new(),
        };

        let sessions = self.sessions.read().await;
        session_ids
            .iter()
            .filter_map(|id| sessions.get(id).cloned())
            .collect()
    }

    pub async fn get_session(&self, session_id: &Uuid) -> Option<Arc<Session>> {
        self.sessions.read().await.get(session_id).cloned()
    }
}
