use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CollaborationMessage {
    // User presence
    UserJoined {
        user_id: Uuid,
        notebook_id: Uuid,
    },
    UserLeft {
        user_id: Uuid,
        notebook_id: Uuid,
    },
    
    // Cursor and selection
    CursorMove {
        user_id: Uuid,
        notebook_id: Uuid,
        position: CursorPosition,
    },
    
    // Content updates
    ContentUpdate {
        user_id: Uuid,
        notebook_id: Uuid,
        content: String,
    },
    
    // Operational transforms
    Operation {
        user_id: Uuid,
        notebook_id: Uuid,
        operation: Operation,
    },
    
    // Sync requests
    SyncRequest {
        user_id: Uuid,
        notebook_id: Uuid,
        version: u64,
    },
    
    // Sync response
    SyncResponse {
        notebook_id: Uuid,
        content: String,
        version: u64,
        operations: Vec<Operation>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: u32,
    pub column: u32,
    pub selection_start: Option<(u32, u32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Insert {
        position: usize,
        text: String,
    },
    Delete {
        position: usize,
        length: usize,
    },
    Retain {
        length: usize,
    },
}
