//! SQLite Database Module
//!
//! Handles persistent storage for sessions and messages.

use std::sync::OnceLock;
use tokio::sync::Mutex;
use anyhow::Result;

static DATABASE: OnceLock<Mutex<Option<Database>>> = OnceLock::new();

pub struct Database {
    // TODO: Add rusqlite connection
    _initialized: bool,
}

pub struct DatabaseConfig {
    pub path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "./data/assistant.db".to_string(),
        }
    }
}

/// Initialize the database
pub async fn init() -> Result<()> {
    let config = DatabaseConfig::default();
    println!("Initializing database: {}", config.path);

    // TODO: Initialize SQLite connection
    // TODO: Run migrations

    let db = Database {
        _initialized: true,
    };

    DATABASE.get_or_init(|| Mutex::new(Some(db)));
    println!("Database initialized successfully");
    Ok(())
}

/// Check if database is initialized
pub fn is_initialized() -> bool {
    DATABASE.get()
        .map(|m| m.try_lock().map(|g| g.is_some()).unwrap_or(false))
        .unwrap_or(false)
}

/// SQL schema for sessions table
pub const SESSIONS_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
"#;

/// SQL schema for messages table
pub const MESSAGES_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id);
"#;
