//! Chat Message Model

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a chat message in a conversation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: ChatRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub fn new(session_id: Uuid, role: ChatRole, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            role,
            content,
            created_at: Utc::now(),
        }
    }

    pub fn user(session_id: Uuid, content: String) -> Self {
        Self::new(session_id, ChatRole::User, content)
    }

    pub fn assistant(session_id: Uuid, content: String) -> Self {
        Self::new(session_id, ChatRole::Assistant, content)
    }

    pub fn system(session_id: Uuid, content: String) -> Self {
        Self::new(session_id, ChatRole::System, content)
    }
}

/// Role of a chat message sender
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
            ChatRole::System => write!(f, "system"),
        }
    }
}
