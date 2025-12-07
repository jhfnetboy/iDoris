//! Session Model

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a chat session
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn default_title() -> Self {
        Self::new("New Chat".to_string())
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::default_title()
    }
}
