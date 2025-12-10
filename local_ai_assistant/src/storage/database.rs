//! SQLite Database Module
//!
//! Handles persistent storage for sessions and messages.

use std::sync::OnceLock;
use tokio::sync::Mutex;
use anyhow::Result;
use rusqlite::Connection;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::{Session, ChatMessage, ChatRole};

static DATABASE: OnceLock<Mutex<Connection>> = OnceLock::new();

/// Get the project root directory (where Cargo.toml is)
fn get_project_root() -> std::path::PathBuf {
    // Try to find the project root by looking for Cargo.toml
    let mut path = std::env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    // Walk up the directory tree looking for Cargo.toml or use a fixed path
    for _ in 0..10 {
        if path.join("Cargo.toml").exists() {
            return path;
        }
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            break;
        }
    }

    // Fallback to the local_ai_assistant project directory
    let fallback = std::path::PathBuf::from("/Volumes/UltraDisk/Dev2/crypto-projects/AI-test/local_ai_assistant");
    if fallback.exists() {
        return fallback;
    }

    // Last resort: use current directory
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}

/// Initialize the database
pub async fn init() -> Result<()> {
    // Use project root for data directory
    let project_root = get_project_root();
    let data_dir = project_root.join("data");

    // Create data directory if it doesn't exist
    std::fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("assistant.db");
    println!("Initializing database: {:?}", db_path);

    let conn = Connection::open(db_path)?;

    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id)",
        [],
    )?;

    DATABASE.get_or_init(|| Mutex::new(conn));
    println!("Database initialized successfully");
    Ok(())
}

/// Check if database is initialized
pub fn is_initialized() -> bool {
    DATABASE.get().is_some()
}

/// Get database connection
fn get_db() -> Option<&'static Mutex<Connection>> {
    DATABASE.get()
}

/// Create a new session
pub async fn create_session(session: &Session) -> Result<()> {
    let db = get_db().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    let conn = db.lock().await;

    conn.execute(
        "INSERT INTO sessions (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        [
            &session.id.to_string(),
            &session.title,
            &session.created_at.to_rfc3339(),
            &session.updated_at.to_rfc3339(),
        ],
    )?;

    Ok(())
}

/// Get all sessions ordered by updated_at desc
pub async fn get_all_sessions() -> Result<Vec<Session>> {
    let db = get_db().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    let conn = db.lock().await;

    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM sessions ORDER BY updated_at DESC"
    )?;

    let sessions = stmt.query_map([], |row| {
        let id_str: String = row.get(0)?;
        let title: String = row.get(1)?;
        let created_at_str: String = row.get(2)?;
        let updated_at_str: String = row.get(3)?;

        Ok((id_str, title, created_at_str, updated_at_str))
    })?
    .filter_map(|r| r.ok())
    .filter_map(|(id_str, title, created_at_str, updated_at_str)| {
        let id = Uuid::parse_str(&id_str).ok()?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str).ok()?.with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str).ok()?.with_timezone(&Utc);

        Some(Session { id, title, created_at, updated_at })
    })
    .collect();

    Ok(sessions)
}

/// Update session title
pub async fn update_session_title(session_id: Uuid, title: &str) -> Result<()> {
    let db = get_db().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    let conn = db.lock().await;

    conn.execute(
        "UPDATE sessions SET title = ?1, updated_at = ?2 WHERE id = ?3",
        [
            title,
            &Utc::now().to_rfc3339(),
            &session_id.to_string(),
        ],
    )?;

    Ok(())
}

/// Delete a session and all its messages
pub async fn delete_session(session_id: Uuid) -> Result<()> {
    let db = get_db().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    let conn = db.lock().await;

    // Delete messages first
    conn.execute(
        "DELETE FROM messages WHERE session_id = ?1",
        [&session_id.to_string()],
    )?;

    // Delete session
    conn.execute(
        "DELETE FROM sessions WHERE id = ?1",
        [&session_id.to_string()],
    )?;

    Ok(())
}

/// Save a message
pub async fn save_message(message: &ChatMessage) -> Result<()> {
    let db = get_db().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    let conn = db.lock().await;

    let role_str = match message.role {
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::System => "system",
    };

    conn.execute(
        "INSERT OR REPLACE INTO messages (id, session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            &message.id.to_string(),
            &message.session_id.to_string(),
            role_str,
            &message.content,
            &message.created_at.to_rfc3339(),
        ],
    )?;

    // Update session's updated_at
    conn.execute(
        "UPDATE sessions SET updated_at = ?1 WHERE id = ?2",
        [
            &Utc::now().to_rfc3339(),
            &message.session_id.to_string(),
        ],
    )?;

    Ok(())
}

/// Get all messages for a session
pub async fn get_session_messages(session_id: Uuid) -> Result<Vec<ChatMessage>> {
    let db = get_db().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    let conn = db.lock().await;

    let mut stmt = conn.prepare(
        "SELECT id, session_id, role, content, created_at FROM messages WHERE session_id = ?1 ORDER BY created_at ASC"
    )?;

    let messages = stmt.query_map([&session_id.to_string()], |row| {
        let id_str: String = row.get(0)?;
        let session_id_str: String = row.get(1)?;
        let role_str: String = row.get(2)?;
        let content: String = row.get(3)?;
        let created_at_str: String = row.get(4)?;

        Ok((id_str, session_id_str, role_str, content, created_at_str))
    })?
    .filter_map(|r| r.ok())
    .filter_map(|(id_str, session_id_str, role_str, content, created_at_str)| {
        let id = Uuid::parse_str(&id_str).ok()?;
        let session_id = Uuid::parse_str(&session_id_str).ok()?;
        let role = match role_str.as_str() {
            "user" => ChatRole::User,
            "assistant" => ChatRole::Assistant,
            "system" => ChatRole::System,
            _ => return None,
        };
        let created_at = DateTime::parse_from_rfc3339(&created_at_str).ok()?.with_timezone(&Utc);

        Some(ChatMessage { id, session_id, role, content, created_at })
    })
    .collect();

    Ok(messages)
}
